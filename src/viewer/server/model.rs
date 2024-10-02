use core::{f32, panic, str};
use std::{
    collections::{HashMap, LinkedList, VecDeque},
    path::Path,
    sync::Arc,
};

use glam::{vec3, Vec3};
use ordered_float::OrderedFloat;
use parking_lot::RwLock;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    vec,
};
use rether::{
    alloc::{AllocHandle, DynamicAllocHandle, ModifyAction},
    model::{
        self, geometry::Geometry, BufferLocation, Model, ModelState, RotateModel, ScaleModel,
        TranslateModel, TreeModel,
    },
    picking::{interact::InteractiveModel, Hitbox, HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer, Rotate, SimpleGeometry,
};

use stl_io::IndexedTriangle;
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
    ui::{api::trim_text, custom_toasts::MODEL_LOAD_PROGRESS},
    viewer::tracker::Process,
    GlobalState, RootEvent, GLOBAL_STATE,
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
    process: Arc<Process>,
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

            let global_state = GLOBAL_STATE.read();
            let global_state = global_state.as_ref().unwrap();

            let process_tracking = global_state
                .progress_tracker
                .write()
                .add(MODEL_LOAD_PROGRESS, trim_text::<20, 4>(&path));

            let vertices: Vec<Vec3> = stl_model
                .vertices
                .iter()
                .map(|vertex| {
                    Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                        *vertex,
                    ))
                })
                .collect();

            process_tracking.set_task("Clustering models".to_string());
            process_tracking.set_progress(0.0);
            let models = clusterize_models(&stl_model.faces);

            process_tracking.set_task("Creating vertices".to_string());
            process_tracking.set_progress(0.2);
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

            process_tracking.set_task("Clustering faces".to_string());
            process_tracking.set_progress(0.4);
            let plane_entries = clusterize_faces(&stl_model.faces, &vertices);

            process_tracking.set_task("Creating polygons".to_string());
            process_tracking.set_progress(0.6);
            let polygons: Vec<PolygonFace> = plane_entries
                .iter()
                .map(|entry| PolygonFace::from_entry(entry.clone(), &stl_model.faces, &vertices))
                .collect();

            let mut triangle_vertices = vec3s_into_vertices(vertices.clone(), Color::BLACK);

            process_tracking.set_task("Filtering polygons".to_string());
            process_tracking.set_progress(0.8);
            let polygon_faces: Vec<PolygonFace> = polygons
                .into_iter()
                .filter(|face| {
                    let x = face.max.x - face.min.x;
                    let y = face.max.y - face.min.y;
                    let z = face.max.z - face.min.z;

                    if x < y && x < z {
                        z > 2.0 && y > 2.0
                    } else if y < x && y < z {
                        x > 2.0 && z > 2.0
                    } else {
                        x > 2.0 && y > 2.0
                    }
                })
                .collect();

            process_tracking.set_task("Coloring polygons".to_string());
            process_tracking.set_progress(0.85);
            models.iter().for_each(|indices| {
                let r = rand::random::<f64>();
                let g = rand::random::<f64>();
                let b = rand::random::<f64>();

                for triangle in indices.iter() {
                    stl_model.faces[*triangle]
                        .vertices
                        .iter()
                        .for_each(|index| {
                            triangle_vertices[*index].color = Color { r, g, b, a: 1.0 }.to_array();
                        });
                }
            });

            let plane_len = plane_entries.len();

            process_tracking.set_task("Creating models".to_string());
            process_tracking.set_progress(0.9);
            let models = polygon_faces
                .clone()
                .into_par_iter()
                .fold(
                    || Vec::with_capacity(plane_len),
                    |mut models, face| {
                        let model = TreeModel::create_node(BufferLocation { offset: 0, size: 0 });
                        models.push(CADModel::Face {
                            model,
                            face: RwLock::new(face),
                        });

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

            process_tracking.set_task("Rotating model to largest face".to_string());
            process_tracking.set_progress(0.95);
            let bounding_box = match &root {
                CADModel::Root { bounding_box, .. } => bounding_box,
                _ => panic!("Not root"),
            };

            let center = bounding_box.center();
            root.translate(-vec3(center.x, center.y, center.z));

            tx.send(Ok(CADModelHandle {
                process: process_tracking,
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

        // model_handle.process.set_task("Finding Name".to_string());
        let mut name = file_name.clone();

        let mut counter: u8 = 1;

        while self.models.contains_key(&name) {
            name = format!("{} ({counter})", file_name);

            counter += 1;
        }

        model_handle.process.set_task("Write to GPU".to_string());
        model_handle.process.set_progress(1.0);
        let handle = {
            let model_state = &*model_handle.model.state().read();

            let data = match model_state {
                rether::model::ModelState::Dormant(geometry) => geometry.build_data(),
                _ => panic!("Unsupported geometry"),
            };

            self.buffer
                .allocate_init(&name, data, &wgpu_context.device, &wgpu_context.queue)
        };

        model_handle.process.finish();

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
        face: RwLock<PolygonFace>,
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
                Self::Face { face, .. } => (face.read().min, face.read().max),
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
            Self::Face { face, .. } => face.read().check_hit(ray),
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
            Self::Face { face, .. } => face.read().min,
        }
    }

    fn get_max(&self) -> Vec3 {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.max,
            Self::Face { face, .. } => face.read().max,
        }
    }
}

impl InteractiveModel for CADModel {
    fn clicked(&self, event: rether::picking::interact::ClickEvent) {
        match self {
            Self::Root { .. } => println!("Root clicked"),
            Self::Face { model, face } => {
                let handle = match &*model.state().read() {
                    rether::model::ModelState::Awake(handle) => handle.clone(),
                    _ => panic!("Not awake"),
                };

                let indices = face.read().indices.clone();

                let mod_action = Box::new(move |data: &mut [Vertex]| {
                    for index in indices.iter() {
                        data[*index].color = Color::RED.to_array();
                    }
                });

                let action = ModifyAction::new(0, handle.size(), mod_action);

                handle.send_action(action).expect("Failed to send action");
            }
        }
    }

    fn drag(&self, event: rether::picking::interact::DragEvent) {
        println!("Clicked");
    }

    fn scroll(&self, event: rether::picking::interact::ScrollEvent) {
        println!("Clicked");
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
            Self::Root {
                model,
                bounding_box,
            } => model.scale(scale, Some(bounding_box.center())),
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
            Self::Root {
                model,
                bounding_box,
            } => {
                model.rotate(rotation, Some(bounding_box.center()));
            }
            Self::Face { model, face } => {
                model.rotate(rotation, center);
                face.write().rotate(rotation, center.unwrap_or(Vec3::ZERO));
            }
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
    triangles: Vec<usize>,
}

impl PartialEq for PlaneEntry {
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane
    }
}

impl Eq for PlaneEntry {}

fn clusterize_models(faces: &[IndexedTriangle]) -> Vec<Vec<usize>> {
    let mut neighbor_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (index, triangle) in faces.iter().enumerate() {
        let t1 = triangle.vertices[0];
        let t2 = triangle.vertices[1];
        let t3 = triangle.vertices[2];

        let mut handle = |t1, t2| {
            if let Some(neighbors) = neighbor_map.get_mut(&(t1, t2)) {
                neighbors.push(index);
            } else if let Some(neighbors) = neighbor_map.get_mut(&(t2, t1)) {
                neighbors.push(index);
            } else {
                neighbor_map.insert((t1, t2), vec![index]);
            }
        };

        handle(t1, t2);
        handle(t2, t3);
        handle(t3, t1);
    }

    let mut visited = vec![false; faces.len()];

    let mut model_faces: LinkedList<Vec<usize>> = LinkedList::new();
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[0] = true;
    model_faces.push_back(vec![0]);
    queue.push_back(0);

    while let Some(index) = queue.pop_front() {
        let triangle = &faces[index];

        let mut handle_edge = |t1, t2| {
            if let Some(neighbors) = neighbor_map.get(&(t1, t2)) {
                for neighbor in neighbors {
                    if !visited[*neighbor] {
                        visited[*neighbor] = true;

                        model_faces.back_mut().unwrap().push(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            } else if let Some(neighbors) = neighbor_map.get(&(t2, t1)) {
                for neighbor in neighbors {
                    if !visited[*neighbor] {
                        visited[*neighbor] = true;

                        model_faces.back_mut().unwrap().push(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            }
        };

        handle_edge(triangle.vertices[0], triangle.vertices[1]);
        handle_edge(triangle.vertices[1], triangle.vertices[2]);
        handle_edge(triangle.vertices[2], triangle.vertices[0]);

        if queue.is_empty() {
            if let Some(index) = (0..faces.len()).find(|index| !visited[*index]) {
                visited[index] = true;
                model_faces.push_back(vec![index]);
                queue.push_back(index);
            }
        }
    }

    model_faces.into_iter().collect()
}

fn clusterize_faces(faces: &[IndexedTriangle], vertices: &[Vec3]) -> Vec<PlaneEntry> {
    let mut plane_map: HashMap<[OrderedFloat<f32>; 6], Vec<usize>> = HashMap::new();

    for (index, triangle) in faces.iter().enumerate() {
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
            let factor = 10f32.powi(4); // 10^4 = 10000
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

        plane_map.entry(key).or_default().push(index);
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

#[derive(Debug, Clone)]
pub struct PolygonFace {
    plane: Plane,
    indices: Vec<usize>,
    min: Vec3,
    max: Vec3,
}

impl PolygonFace {
    fn from_entry(entry: PlaneEntry, faces: &[IndexedTriangle], vertices: &[Vec3]) -> PolygonFace {
        let plane = Plane {
            normal: (vertices[faces[entry.triangles[0]].vertices[1]]
                - vertices[faces[entry.triangles[0]].vertices[0]])
                .cross(
                    vertices[faces[entry.triangles[0]].vertices[2]]
                        - vertices[faces[entry.triangles[0]].vertices[0]],
                )
                .normalize(),
            point: vertices[faces[entry.triangles[0]].vertices[0]],
        };

        let mut min = Vec3::INFINITY;
        let mut max = Vec3::NEG_INFINITY;

        for triangle in entry.triangles.iter() {
            min = min
                .min(vertices[faces[*triangle].vertices[0]])
                .min(vertices[faces[*triangle].vertices[1]])
                .min(vertices[faces[*triangle].vertices[2]]);
            max = max
                .max(vertices[faces[*triangle].vertices[0]])
                .max(vertices[faces[*triangle].vertices[1]])
                .max(vertices[faces[*triangle].vertices[2]]);
        }

        let indices = entry
            .triangles
            .iter()
            .flat_map(|index| {
                let triangle = &faces[*index];

                vec![
                    triangle.vertices[0],
                    triangle.vertices[1],
                    triangle.vertices[2],
                ]
            })
            .collect();

        Self {
            plane,
            indices,
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

        if intersection.x > self.min.x
            && intersection.x < self.max.x
            && intersection.y > self.min.y
            && intersection.y < self.max.y
            && intersection.z > self.min.z
            && intersection.z < self.max.z
        {
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

impl Rotate for PolygonFace {
    fn rotate(&mut self, rotation: glam::Quat, center: Vec3) {
        self.plane.point = rotation * (self.plane.point - center) + center;

        self.min = rotation * (self.min - center) + center;
        self.max = rotation * (self.max - center) + center;
    }
}
