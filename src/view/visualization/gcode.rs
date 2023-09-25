use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use bevy::prelude::Mesh;
use bevy::render::render_resource::PrimitiveTopology;
use three_d_asset::Srgba;
use three_d_asset::TriMesh;

use crate::model::gcode::toolpath::compute_modul_with_coordinator;
use crate::model::gcode::toolpath::PathModul;
use crate::model::gcode::toolpath::ToolPath;
use crate::model::gcode::GCode;
use crate::model::layer::LayerMesh;
use crate::model::layer::PartCoordinator;
use crate::model::layer::ToolPathModel;
use crate::utils::debug::DebugWrapper;
use crate::utils::task::TaskWithResult;

struct MeshWrapper(TriMesh);

impl std::fmt::Debug for MeshWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_f = f.debug_struct("Mesh");

        debug_f.field("positions", &DebugWrapper::from(self.0.positions.to_f64()));

        if let Some(indices) = self.0.indices.to_u32() {
            debug_f.field("indices", &DebugWrapper::from(indices));
        }

        if let Some(uvs) = &self.0.uvs {
            debug_f.field("uvs", &DebugWrapper::from(uvs));
        }

        debug_f.finish()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Layer {
    mesh: MeshWrapper,
    color: Srgba,
}

#[allow(dead_code)]
impl Layer {
    fn triangle_mesh(&self) -> &TriMesh {
        &self.mesh.0
    }
}

#[allow(dead_code)]
#[derive()]
pub struct GCodeVisualizer {
    gcode: Arc<Mutex<Cell<Option<crate::model::gcode::GCode>>>>,
    result: Option<Arc<Mutex<TaskWithResult<Vec<Layer>>>>>,
}

impl Default for GCodeVisualizer {
    fn default() -> Self {
        Self {
            gcode: Arc::new(Mutex::new(Cell::new(None))),
            result: None,
        }
    }
}

#[allow(dead_code)]
impl GCodeVisualizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cell(&self) -> Arc<Mutex<Cell<Option<crate::model::gcode::GCode>>>> {
        self.gcode.clone()
    }

    fn gcode_exists(&self) -> bool {
        self.gcode.lock().unwrap().get_mut().is_some()
    }

    pub fn gcode_needs_computing(&self) -> bool {
        self.gcode_exists()
            && !self
                .gcode
                .lock()
                .unwrap()
                .get_mut()
                .as_ref()
                .unwrap()
                .rendered
    }

    pub fn build_gcode_model<'a>(&self) -> ToolPathModel<'a> {
        let toolpath = ToolPath::from(
            self.gcode
                .lock()
                .unwrap()
                .get_mut()
                .as_ref()
                .unwrap()
                .clone(),
        );
        let modul_map: HashMap<usize, Vec<PathModul>> = toolpath.into();

        self.gcode
            .lock()
            .unwrap()
            .get_mut()
            .as_mut()
            .unwrap()
            .rendered = true;

        let mut layers: HashMap<usize, RefCell<LayerMesh<'a>>> = HashMap::new();

        for entry in modul_map.iter() {
            let layer = LayerMesh::empty();
            layers.insert(*entry.0, RefCell::new(layer));
        }

        unsafe {
            for entry in modul_map.into_iter() {
                let layer = layers.get(&entry.0).unwrap();
                let coordinator = PartCoordinator::new(layer.as_ptr().as_mut().unwrap());

                for modul in entry.1 {
                    compute_modul_with_coordinator(&modul, &coordinator);
                }
            }
        }

        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut normals = Vec::new();

        for entry in layers.iter() {
            let mut layer = entry.1.borrow_mut();

            positions.append(&mut layer.mesh.positions);
            colors.append(&mut layer.mesh.colors);
            normals.append(&mut layer.mesh.normals);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.compute_flat_normals();

        ToolPathModel { layers, mesh }
    }
}

pub fn create_toolpath<'a>(gcode: &GCode) -> ToolPathModel<'a> {
    let toolpath = ToolPath::from(gcode.clone());
    let modul_map: HashMap<usize, Vec<PathModul>> = toolpath.into();

    let mut layers: HashMap<usize, RefCell<LayerMesh<'a>>> = HashMap::new();

    for entry in modul_map.iter() {
        let layer = LayerMesh::empty();
        layers.insert(*entry.0, RefCell::new(layer));
    }

    unsafe {
        for entry in modul_map.into_iter() {
            let layer = layers.get(&entry.0).unwrap();
            let coordinator = PartCoordinator::new(layer.as_ptr().as_mut().unwrap());

            for modul in entry.1 {
                compute_modul_with_coordinator(&modul, &coordinator);
            }
        }
    }

    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut normals = Vec::new();

    for entry in layers.iter() {
        let mut layer = entry.1.borrow_mut();

        positions.append(&mut layer.mesh.positions);
        colors.append(&mut layer.mesh.colors);
        normals.append(&mut layer.mesh.normals);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    //mesh.set_indices(Some(Indices::U32(
    //    (0..colors.len()).map(|e| e as u32).collect(),
    //)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    ToolPathModel { layers, mesh }
}

/*
impl GCodeVisualizer {
    pub fn try_collect_objects<'a>(
        &self,
        context: &Context,
    ) -> Result<ToolPathModel<'a>, crate::error::Error> {
        let mut toolpath_model = build_test_meshes(context);

        toolpath_model.model.set_transformation(
            Mat4::from_translation(vec3(-125.0, 5.0, 125.0))
                .concat(&Mat4::from_angle_x(degrees(-90.0))),
        );
        //model.set_transformation(Mat4::from_translation(vec3(0.0, 40.0, 0.0)));

        Ok(toolpath_model)
    }
}*/
