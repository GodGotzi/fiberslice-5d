use core::panic;
use std::{
    collections::{HashMap, LinkedList},
    path::Path,
    sync::Arc,
};

use glam::{vec3, Vec2, Vec3};
use rether::{
    alloc::DynamicAllocHandle,
    model::{geometry::Geometry, Model, TreeModel},
    picking::{Hitbox, HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer,
};

use stl_io::{IndexedMesh, Vector};
use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};

use uni_path::PathBuf;

use crate::{geometry::BoundingBox, prelude::WgpuContext, GlobalState, RootEvent};

use crate::viewer::gcode::{tree::ToolpathTree, Toolpath};

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
    handle: Arc<CADModel>,
}

// TODO also use vertex indices
#[derive(Debug)]
pub struct CADModelServer {
    queue: Vec<(Receiver<Toolpath>, JoinHandle<()>)>,

    buffer: rether::Buffer<Vertex, rether::alloc::BufferDynamicAllocator<Vertex>>,

    root_hitbox: HitboxRoot<ToolpathTree>,

    models: HashMap<String, CADModelHandle>,
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
        let content = std::fs::read_to_string(&path).unwrap();
        let path = path.as_ref().to_str().unwrap_or("").to_string();
        let (tx, rx) = tokio::sync::oneshot::channel();

        let handle = tokio::spawn(async move {
            let file = match std::fs::File::open(&path) {
                Ok(file) => file,
                Err(e) => {
                    tx.send(Err(CADModelError::FileNotFound(path))).unwrap();
                    return;
                }
            };

            let mut reader = std::io::BufReader::new(file);
            let stl_model = match stl_io::read_stl(&mut reader) {
                Ok(stl_model) => stl_model,
                Err(e) => {
                    tx.send(Err(CADModelError::LoadError(path))).unwrap();
                    return;
                }
            };

            let asd = stl_model.clusterize_faces();
        });

        self.queue.push((rx, handle));
    }
    // i love you
    pub fn insert(
        &mut self,
        part: Toolpath,
        wgpu_context: &WgpuContext,
    ) -> Result<Arc<ToolpathTree>, Error> {
        let path: PathBuf = part.origin_path.into();
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
            let model_state = &*part.model.state().read();

            let data = match model_state {
                rether::model::ModelState::Dormant(geometry) => geometry.build_data(),
                _ => panic!("Unsupported geometry"),
            };

            self.buffer
                .allocate_init(&name, data, &wgpu_context.device, &wgpu_context.queue)
        };

        part.model.wake(handle.clone());

        let code = part.raw.join("\n");

        let line_breaks = code
            .char_indices()
            .filter_map(|(index, c)| if c == '\n' { Some(index) } else { None })
            .collect::<Vec<usize>>();

        let handle = Arc::new(part.model);

        self.models.insert(name.clone());

        self.focused = Some(name.clone());

        Ok(handle.clone())
    }

    pub fn remove(&mut self, name: String, wgpu_context: &WgpuContext) {
        if let Some(model) = self.models.remove(&name) {
            let state = model.handle.state();

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

            for toolpath in results {
                let handle = self.insert(toolpath, wgpu_context)?;

                global_state
                    .ui_event_writer
                    .send(crate::ui::UiEvent::ShowSuccess("Gcode loaded".to_string()));

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

    pub fn root_hitbox(&self) -> &HitboxRoot<ToolpathTree> {
        &self.root_hitbox
    }
}

#[derive(Debug)]
pub enum CADModel {
    Root {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
    },
    Face {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
    },
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

#[derive(Debug)]
struct Plane {
    normal: glam::Vec3,
    point: glam::Vec3,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        // check if the planes are mathematically equal

        let cross_product = self.normal.cross(other.normal);
        if !cross_product.is_nan() {
            return false; // Normals are not parallel
        }

        // Step 2: Check if p2 lies on the first plane and p1 lies on the second plane
        let p2_on_plane1 = (vec3(
            other.point.x - self.point.x,
            other.point.y - self.point.y,
            other.point.z - self.point.z,
        ))
        .dot(self.normal)
        .abs()
            < f32::EPSILON;

        let p1_on_plane2 = (vec3(
            self.point.x - other.point.x,
            self.point.y - other.point.y,
            self.point.z - other.point.z,
        ))
        .dot(other.normal)
        .abs()
            < f32::EPSILON;

        p2_on_plane1 && p1_on_plane2
    }
}

impl Eq for Plane {}

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

trait ClusterizeFaces {
    fn clusterize_faces(&self) -> Vec<PlaneEntry>;
}

impl ClusterizeFaces for IndexedMesh {
    fn clusterize_faces(&self) -> Vec<PlaneEntry> {
        let mut planes: Vec<PlaneEntry> = Vec::new();

        for face in self.faces.iter() {
            let v0 = self.vertices[face.vertices[0]];

            let plane = Plane {
                normal: Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                    face.normal,
                ))
                .normalize(),
                point: Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                    v0,
                )),
            };

            if let Some(entry) = planes.iter_mut().find(|entry| entry.plane == plane) {
                entry.triangles.push(face.vertices);
            } else {
                planes.push(PlaneEntry {
                    plane,
                    triangles: vec![face.vertices],
                });
            }
        }

        planes
    }
}

#[derive(Debug)]
pub struct PolygonFace {
    plane: Plane,
    strokes: Vec<Stroke>,
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
    fn from_entry(entry: PlaneEntry, vertices: &[Vector<f32>]) -> Self {
        let triangles: Vec<[Vec3; 3]> = entry
            .triangles
            .iter()
            .map(|index| {
                [
                    Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                        vertices[index[0]],
                    )),
                    Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                        vertices[index[1]],
                    )),
                    Vec3::from(<stl_io::Vector<f32> as std::convert::Into<[f32; 3]>>::into(
                        vertices[index[2]],
                    )),
                ]
            })
            .collect();

        let strokes = determine_contour(&triangles);
        let min = strokes
            .iter()
            .fold(Vec3::splat(f32::INFINITY), |min, stroke| {
                min.min(stroke.0.min(stroke.1))
            });

        let max = strokes
            .iter()
            .fold(Vec3::splat(f32::NEG_INFINITY), |max, stroke| {
                max.max(stroke.0.max(stroke.1))
            });

        Self {
            plane: entry.plane,
            strokes,
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
            let normal = edge.cross(ray_dir).normalize();

            let to_intersection = intersection - stroke.0;

            // Compute cross product of edge and vector to the intersection point
            let cross = edge.cross(to_intersection).dot(self.plane.normal);

            if cross.abs() < f32::EPSILON {
                continue; // The point lies exactly on the edge, treat as inside
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

struct StrokeEntry(Stroke, usize);

fn determine_contour(vertices: &Vec<[Vec3; 3]>) -> Vec<Stroke> {
    let mut strokes: Vec<StrokeEntry> = Vec::new();

    for triangle in vertices {
        let mut stroke = Stroke(triangle[0], triangle[1]);

        if let Some(entry) = strokes.iter_mut().find(|entry| entry.0 == stroke) {
            entry.1 += 1;
        } else {
            strokes.push(StrokeEntry(stroke, 1));
        }

        stroke = Stroke(triangle[1], triangle[2]);

        if let Some(entry) = strokes.iter_mut().find(|entry| entry.0 == stroke) {
            entry.1 += 1;
        } else {
            strokes.push(StrokeEntry(stroke, 1));
        }

        stroke = Stroke(triangle[2], triangle[0]);

        if let Some(entry) = strokes.iter_mut().find(|entry| entry.0 == stroke) {
            entry.1 += 1;
        } else {
            strokes.push(StrokeEntry(stroke, 1));
        }
    }

    let contour_strokes: Vec<Stroke> = strokes
        .into_iter()
        .filter_map(|entry| if entry.1 == 1 { Some(entry.0) } else { None })
        .collect();

    contour_strokes
}
