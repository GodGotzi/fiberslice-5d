use core::{f32, panic};
use std::{
    collections::{HashMap, LinkedList, VecDeque},
    path::Path,
    sync::Arc,
};

use glam::{vec3, Vec3};
use image::imageops::FilterType::Triangle;
use ordered_float::OrderedFloat;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelDrainRange, ParallelIterator,
};
use rether::{
    alloc::{AllocHandle, DynamicAllocHandle},
    model::{
        geometry::Geometry, BufferLocation, Model, ModelState, RotateModel, ScaleModel,
        TranslateModel, TreeModel,
    },
    picking::{Hitbox, HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer, SimpleGeometry,
};

use stl_io::{IndexedMesh, IndexedTriangle};
use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};

use uni_path::PathBuf;
use wgpu::Color;

use crate::{
    geometry::{
        mesh::{vec3s_into_vertices, IntoArrayColor},
        BoundingBox,
    },
    prelude::WgpuContext,
    GlobalState, RootEvent,
};

// const MAIN_LOADED_TOOLPATH: &str = "main"; // HACK: This is a solution to ease the dev when only one toolpath is loaded which is the only supported(for now)

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Load Error {0}")]
    LoadError(String),
    #[error("NoGeometryObject")]
    NoGeometryObject,
}

#[derive(Debug)]
pub struct CADModelHandle {
    model: CADModel,
    origin_path: String,
}

type CADModelResult = Result<CADModelHandle, CADModelError>;

// TODO also use vertex indices
#[derive(Debug)]
pub struct CADModelServer {
    queue: Vec<(Receiver<CADModelResult>, JoinHandle<()>)>,

    buffer: rether::Buffer<Vertex, rether::alloc::BufferDynamicAllocator<Vertex>>,

    root_hitbox: HitboxRoot<CADModel>,

    models: HashMap<String, Arc<CADModel>>,
    focused: Option<String>,
}

impl CADModelServer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            queue: Vec::new(),
            buffer: Buffer::new("CADModel Buffer", device),
            root_hitbox: HitboxRoot::root(),
            models: HashMap::new(),
            focused: None,
        }
    }

    pub fn load<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str().unwrap_or("").to_string();
        let (tx, rx) = tokio::sync::oneshot::channel();

        let handle = tokio::spawn(async move {
            let file = match std::fs::File::open(&path) {
                Ok(file) => file,
                Err(_e) => {
                    tx.send(Err(CADModelError::FileNotFound(path))).unwrap();
                    return;
                }
            };

            let mut reader = std::io::BufReader::new(file);
            let mut stl_model = match stl_io::read_stl(&mut reader) {
                Ok(stl_model) => stl_model,
                Err(_e) => {
                    tx.send(Err(CADModelError::LoadError(path))).unwrap();
                    return;
                }
            };

            println!("Model loaded 1");

            println!("Model loaded 1.5");

            let vertices: Vec<Vec3> = stl_model
                .vertices
                .iter()
                .map(|vertex| {
                    Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                        *vertex,
                    ))
                })
                .collect();

            println!("Model loaded 2");

            let vertices = stl_model
                .faces
                .iter_mut()
                .fold(Vec::new(), |mut vec, face| {
                    vec.push(vertices[face.vertices[0]]);
                    face.vertices[0] = vec.len() - 1;
                    vec.push(vertices[face.vertices[1]]);
                    face.vertices[1] = vec.len() - 1;
                    vec.push(vertices[face.vertices[2]]);
                    face.vertices[2] = vec.len() - 1;
                    vec
                });

            let plane_entries = clusterize_faces(&stl_model.faces, &vertices);

            let planes = plane_entries
                .iter()
                .map(|entry| PolygonFace::from_entry(entry, &vertices))
                .collect::<Vec<PolygonFace>>();

            let mut triangle_vertices = vec3s_into_vertices(vertices.clone(), Color::BLACK);

            let plane_entries: Vec<PlaneEntry> = plane_entries
                .into_iter()
                .zip(planes.iter())
                .filter(|(_, polygon)| {
                    println!("max_circle: {:?}", polygon.max_circle_radius);

                    polygon.max_circle_radius > 2.0
                })
                .map(|(entry, _)| entry)
                .collect();

            println!("entries: {:?}", plane_entries.len());

            plane_entries.iter().for_each(|entry| {
                let r = rand::random::<f64>();
                let g = rand::random::<f64>();
                let b = rand::random::<f64>();

                for index in &entry.triangles {
                    triangle_vertices[index[0]].color = Color { r, g, b, a: 1.0 }.to_array();

                    triangle_vertices[index[1]].color = Color { r, g, b, a: 1.0 }.to_array();

                    triangle_vertices[index[2]].color = Color { r, g, b, a: 1.0 }.to_array();
                }
            });

            let plane_len = planes.len();

            let models = planes
                .into_par_iter()
                .fold(
                    || Vec::with_capacity(plane_len),
                    |mut models, face| {
                        let model = TreeModel::create_node(BufferLocation { offset: 0, size: 0 });
                        models.push(CADModel::Face { model, face });

                        models
                    },
                )
                .reduce(
                    || Vec::with_capacity(plane_len),
                    |mut models, mut models2| {
                        models.append(&mut models2);
                        models
                    },
                );

            let root = CADModel::create_root(SimpleGeometry::init(triangle_vertices), models);

            tx.send(Ok(CADModelHandle {
                model: root,
                origin_path: path,
            }))
            .unwrap();
        });

        self.queue.push((rx, handle));
    }
    // i love you
    pub fn insert(
        &mut self,
        model_handle: CADModelHandle,
        wgpu_context: &WgpuContext,
    ) -> Result<Arc<CADModel>, Error> {
        let path: PathBuf = model_handle.origin_path.into();
        let file_name = if let Some(path) = path.file_name() {
            path.to_string()
        } else {
            path.to_string()
        };

        let mut name = file_name.clone();

        let mut counter: u8 = 1;

        while self.models.contains_key(&name) {
            name = format!("{} ({counter})", file_name);

            counter += 1;
        }

        let handle = {
            let model_state = &*model_handle.model.state().read();

            let data = match model_state {
                rether::model::ModelState::Dormant(geometry) => geometry.build_data(),
                _ => panic!("Unsupported geometry"),
            };

            self.buffer
                .allocate_init(&name, data, &wgpu_context.device, &wgpu_context.queue)
        };

        model_handle.model.wake(handle.clone());

        let handle = Arc::new(model_handle.model);

        self.models.insert(name.clone(), handle.clone());

        self.focused = Some(name.clone());

        Ok(handle)
    }

    pub fn remove(&mut self, name: String, wgpu_context: &WgpuContext) {
        if let Some(model) = self.models.remove(&name) {
            let state = model.state();

            {
                let handle = match &*state.read() {
                    rether::model::ModelState::Awake(handle) => handle.clone(),
                    rether::model::ModelState::Destroyed => panic!("Already destroyed"),
                    _ => panic!("Not alive"),
                };

                self.buffer
                    .free(handle.id(), &wgpu_context.device, &wgpu_context.queue);
            }
        }
    }

    pub fn update(
        &mut self,
        global_state: GlobalState<RootEvent>,
        wgpu_context: &WgpuContext,
    ) -> Result<(), Error> {
        if !self.queue.is_empty() {
            let mut results = Vec::new();

            self.queue.retain_mut(|(rx, ..)| match rx.try_recv() {
                Ok(result) => {
                    results.push(result);

                    false
                }
                Err(TryRecvError::Closed) => false,
                _ => true,
            });

            for model_result in results {
                let model = match model_result {
                    Ok(model) => model,
                    Err(e) => {
                        global_state
                            .ui_event_writer
                            .send(crate::ui::UiEvent::ShowError(format!("{}", e)));

                        continue;
                    }
                };

                let handle = self.insert(model, wgpu_context)?;

                global_state
                    .ui_event_writer
                    .send(crate::ui::UiEvent::ShowSuccess("Object loaded".to_string()));

                global_state.camera_event_writer.send(
                    crate::camera::CameraEvent::UpdatePreferredDistance(BoundingBox::new(
                        handle.get_min(),
                        handle.get_max(),
                    )),
                );

                let model_trait_handle =
                    handle.clone() as Arc<dyn Model<Vertex, DynamicAllocHandle<Vertex>>>;

                global_state
                    .ui_event_writer
                    .send(crate::ui::UiEvent::ShowProgressBar);

                global_state
                    .viewer
                    .selector()
                    .write()
                    .select(&model_trait_handle);

                self.root_hitbox.add_node(handle);
                // global_state.window.request_redraw();
            }
        }

        self.buffer
            .update(&wgpu_context.device, &wgpu_context.queue);

        Ok(())
    }

    pub fn iter_keys(&self) -> impl Iterator<Item = &String> {
        self.models.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }

    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn kill(&mut self) {
        for (_, handle) in self.queue.drain(..) {
            handle.abort();
        }
    }

    pub fn get_focused(&self) -> Option<&str> {
        self.focused.as_deref()
    }

    pub fn get_focused_mut(&mut self) -> &mut Option<String> {
        &mut self.focused
    }

    pub fn read_buffer(&self) -> &Buffer<Vertex, rether::alloc::BufferDynamicAllocator<Vertex>> {
        &self.buffer
    }

    pub fn root_hitbox(&self) -> &HitboxRoot<CADModel> {
        &self.root_hitbox
    }
}

#[derive(Debug)]
pub enum CADModel {
    Root {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
        bounding_box: BoundingBox,
    },
    Face {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
        face: PolygonFace,
    },
}

impl CADModel {
    pub fn create_root<T: Into<ModelState<Vertex, DynamicAllocHandle<Vertex>>>>(
        geometry: T,
        models: Vec<CADModel>,
    ) -> Self {
        let bounding_box = models.iter().fold(BoundingBox::default(), |mut bb, model| {
            let (min, max) = match model {
                Self::Root { bounding_box, .. } => (bounding_box.min, bounding_box.max),
                Self::Face { face, .. } => (face.min, face.max),
            };

            bb.expand_point(min);
            bb.expand_point(max);
            bb
        });

        Self::Root {
            model: TreeModel::create_root_with_models(geometry, models),
            bounding_box,
        }
    }
}

impl HitboxNode<CADModel> for CADModel {
    fn check_hit(&self, ray: &rether::picking::Ray) -> Option<f32> {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.check_hit(ray),
            Self::Face { face, .. } => face.check_hit(ray),
        }
    }

    fn inner_nodes(&self) -> &[CADModel] {
        match self {
            Self::Root { model, .. } => model.sub_handles().expect("No sub handles"),
            Self::Face { model, .. } => model.sub_handles().expect("No sub handles"),
        }
    }

    fn get_min(&self) -> Vec3 {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.min,
            Self::Face { face, .. } => face.min,
        }
    }

    fn get_max(&self) -> Vec3 {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.max,
            Self::Face { face, .. } => face.max,
        }
    }
}

impl Model<Vertex, DynamicAllocHandle<Vertex>> for CADModel {
    fn wake(&self, handle: Arc<DynamicAllocHandle<Vertex>>) {
        match self {
            Self::Root { model, .. } => model.wake(handle),
            Self::Face { model, .. } => model.wake(handle),
        }
    }

    fn transform(&self) -> rether::Transform {
        match self {
            Self::Root { model, .. } => model.transform(),
            Self::Face { model, .. } => model.transform(),
        }
    }

    fn state(&self) -> &parking_lot::RwLock<ModelState<Vertex, DynamicAllocHandle<Vertex>>> {
        match self {
            Self::Root { model, .. } => model.state(),
            Self::Face { model, .. } => model.state(),
        }
    }
}

impl ScaleModel for CADModel {
    fn scale(&self, scale: Vec3, center: Option<Vec3>) {
        match self {
            Self::Root { model, .. } => model.scale(scale, center),
            Self::Face { model, .. } => model.scale(scale, center),
        }
    }
}

impl TranslateModel for CADModel {
    fn translate(&self, translation: Vec3) {
        match self {
            Self::Root { model, .. } => model.translate(translation),
            Self::Face { model, .. } => model.translate(translation),
        }
    }
}

impl RotateModel for CADModel {
    fn rotate(&self, rotation: glam::Quat, center: Option<Vec3>) {
        match self {
            Self::Root { model, .. } => model.rotate(rotation, center),
            Self::Face { model, .. } => model.rotate(rotation, center),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CADModelError {
    #[error("File not found {0}")]
    FileNotFound(String),
    #[error("Load Error {0}")]
    LoadError(String),
    #[error("NoGeometryObject")]
    NoGeometryObject,
}

#[derive(Debug, Clone)]
struct Plane {
    normal: glam::Vec3,
    point: glam::Vec3,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        // check if the planes are mathematically equal

        let cross_product = self.normal.cross(other.normal);
        if cross_product.length() > f32::EPSILON {
            return false; // Normals are not parallel
        }

        // Step 2: Check if p2 lies on the first plane and p1 lies on the second plane
        (other.point - self.point).dot(self.normal).abs() < f32::EPSILON
    }
}

impl Eq for Plane {}

#[derive(Debug, Clone)]
struct PlaneEntry {
    plane: Plane,
    triangles: Vec<[usize; 3]>,
}

impl PartialEq for PlaneEntry {
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane
    }
}

impl Eq for PlaneEntry {}

fn clusterize_faces(faces: &[IndexedTriangle], vertices: &[Vec3]) -> Vec<PlaneEntry> {
    let mut plane_map: HashMap<[OrderedFloat<f32>; 6], Vec<[usize; 3]>> = HashMap::new();

    for triangle in faces.iter() {
        let normal = Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
            triangle.normal,
        ))
        .normalize();

        let point = vertices[triangle.vertices[0]];

        let ray = rether::picking::Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: normal,
        };

        let intersection = ray.intersection_plane(normal, point);

        fn round(value: f32) -> f32 {
            let factor = 10f32.powi(6); // 10^4 = 10000
            (value * factor).round() / factor
        }

        let key = [
            OrderedFloat(round(normal.x)),
            OrderedFloat(round(normal.y)),
            OrderedFloat(round(normal.z)),
            OrderedFloat(round(intersection.x)),
            OrderedFloat(round(intersection.y)),
            OrderedFloat(round(intersection.z)),
        ];

        plane_map.entry(key).or_default().push(triangle.vertices);
    }

    plane_map
        .into_iter()
        .map(|(key, indices)| {
            let normal = Vec3::new(key[0].0, key[1].0, key[2].0);
            let point = Vec3::new(key[3].0, key[4].0, key[5].0);

            PlaneEntry {
                plane: Plane { normal, point },
                triangles: indices,
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct PolygonFace {
    plane: Plane,
    strokes: Vec<Stroke>,
    max_circle_radius: f32,
    min: Vec3,
    max: Vec3,
}

#[derive(Debug)]
struct Stroke(Vec3, Vec3);

impl PartialEq for Stroke {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance(other.0) < f32::EPSILON && self.1.distance(other.1) < f32::EPSILON
            || self.1.distance(other.0) < f32::EPSILON && self.0.distance(other.1) < f32::EPSILON
    }
}

impl Eq for Stroke {}

impl PolygonFace {
    fn from_entry(entry: &PlaneEntry, vertices: &[Vec3]) -> Self {
        let triangles: Vec<[Vec3; 3]> = entry
            .triangles
            .iter()
            .map(|index| [vertices[index[0]], vertices[index[1]], vertices[index[2]]])
            .collect();

        let strokes = determine_contour(&triangles);

        let mut min = Vec3::INFINITY;
        let mut max = Vec3::NEG_INFINITY;
        let mean = strokes.iter().fold(Vec3::ZERO, |sum, stroke| {
            sum + ((stroke.0 + stroke.1) / 2.0 * (stroke.0 - stroke.1))
        }) / strokes.len() as f32;

        let mut min_radius = f32::INFINITY;

        for stroke in &strokes {
            min = min.min(stroke.0.min(stroke.1));
            max = max.max(stroke.0.max(stroke.1));

            // distance mean to stroke
            let dir = stroke.0 - stroke.1;
            let distance = (mean - stroke.0).cross(dir).length() / dir.length();

            if distance < min_radius {
                min_radius = distance;
            }
        }

        Self {
            plane: entry.plane.clone(),
            strokes,
            max_circle_radius: min_radius,
            min,
            max,
        }
    }
}

impl Hitbox for PolygonFace {
    fn check_hit(&self, ray: &rether::picking::Ray) -> Option<f32> {
        let denominator = self.plane.normal.dot(ray.direction);

        if denominator.abs() < f32::EPSILON {
            return None;
        }

        let t = (self.plane.point - ray.origin).dot(self.plane.normal) / denominator;

        if t < 0.0 {
            return None;
        }

        let intersection = ray.origin + ray.direction * t;

        let ray_dir = self.strokes[0].1 - self.strokes[0].0;

        let mut inside = false;

        for stroke in &self.strokes {
            let edge = stroke.1 - stroke.0;

            let cross_dir = ray_dir.cross(edge).normalize();

            if cross_dir.dot(cross_dir).abs() < f32::EPSILON {
                continue;
            }

            let t1 =
                (stroke.0 - intersection).cross(edge).dot(cross_dir) / cross_dir.dot(cross_dir);

            let t2 =
                (intersection - stroke.0).cross(ray_dir).dot(cross_dir) / cross_dir.dot(cross_dir);

            let intersection_1 = stroke.0 + edge * t1;

            let intersection_2 = intersection + ray_dir * t2;

            if (intersection_1 - intersection_2).length_squared() < f32::EPSILON {
                inside = !inside;
            }
        }

        if inside {
            Some(t)
        } else {
            None
        }
    }

    fn expand_hitbox(&mut self, _box: &dyn Hitbox) {
        panic!("Not implemented")
    }

    fn set_enabled(&mut self, _enabled: bool) {
        panic!("Not implemented")
    }

    fn enabled(&self) -> bool {
        panic!("Not implemented")
    }

    fn get_min(&self) -> Vec3 {
        self.min
    }

    fn get_max(&self) -> Vec3 {
        self.max
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
struct OrderedStroke {
    start: [OrderedFloat<f32>; 3],
    end: [OrderedFloat<f32>; 3],
}

impl From<Stroke> for OrderedStroke {
    fn from(stroke: Stroke) -> Self {
        fn round(value: f32) -> f32 {
            let factor = 10f32.powi(4); // 10^4 = 10000
            (value * factor).round() / factor
        }

        let min = stroke.0.min(stroke.1);
        let max = stroke.0.max(stroke.1);

        Self {
            start: [
                OrderedFloat(round(min.x)),
                OrderedFloat(round(min.y)),
                OrderedFloat(round(min.z)),
            ],
            end: [
                OrderedFloat(round(max.x)),
                OrderedFloat(round(max.y)),
                OrderedFloat(round(max.z)),
            ],
        }
    }
}

impl From<OrderedStroke> for Stroke {
    fn from(stroke: OrderedStroke) -> Self {
        Self(
            Vec3::new(stroke.start[0].0, stroke.start[1].0, stroke.start[2].0),
            Vec3::new(stroke.end[0].0, stroke.end[1].0, stroke.end[2].0),
        )
    }
}

fn determine_contour(vertices: &Vec<[Vec3; 3]>) -> Vec<Stroke> {
    let mut strokes: HashMap<OrderedStroke, usize> = HashMap::new();

    for triangle in vertices {
        let mut stroke = Stroke(triangle[0], triangle[1]);

        strokes
            .entry(stroke.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        stroke = Stroke(triangle[1], triangle[2]);

        strokes
            .entry(stroke.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        stroke = Stroke(triangle[2], triangle[0]);

        strokes
            .entry(stroke.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    strokes
        .into_iter()
        .filter_map(|(stroke, count)| {
            if count == 1 {
                Some(stroke.into())
            } else {
                None
            }
        })
        .collect()
}
