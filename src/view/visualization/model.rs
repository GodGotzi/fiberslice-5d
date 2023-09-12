use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use three_d::*;
use three_d_asset::TriMesh;

use crate::application::Application;
use crate::model::gcode::toolpath::compute_modul_with_coordinator;
use crate::model::gcode::toolpath::PathModul;
use crate::model::gcode::toolpath::ToolPath;
use crate::model::gcode::GCode;
use crate::model::layer::construct_filament_material;
use crate::model::layer::LayerModel;
use crate::model::layer::PartCoordinator;
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
#[derive(Debug, Default)]
pub struct GCodeVisualizer {
    gcode: Option<crate::model::gcode::GCode>,
    result: Option<Arc<Mutex<TaskWithResult<Vec<Layer>>>>>,
}

#[allow(dead_code)]
impl GCodeVisualizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_gcode(&mut self, gcode: crate::model::gcode::GCode) {
        self.gcode = Some(gcode);
    }

    pub fn gcode(&self) -> &Option<crate::model::gcode::GCode> {
        &self.gcode
    }
}

impl GCodeVisualizer {
    fn _visualize(&mut self, application: &mut Application) -> Result<(), crate::error::Error> {
        if self.gcode.is_none() {
            return Err(crate::error::Error::FieldMissing("gcode is missing".into()));
        }

        let mut result = TaskWithResult::<Vec<Layer>>::new();

        let gcode = self.gcode.as_ref().unwrap().clone();

        result.run(Box::new(move || {
            let layers = Vec::new();

            for _instruction in gcode.instruction_moduls.iter() {
                /*

                TODO




                */

                //strokes.push(Stroke { mesh_wrap: MeshWrapper(Mesh::new(context, cpu_mesh)), color: () })
            }

            layers
        }));

        self.result = Some(Arc::new(Mutex::new(result)));

        application
            .task_handler()
            .add_task(self.result.as_ref().unwrap().clone());

        Ok(())
    }

    pub fn try_collect_objects<'a>(
        &self,
        context: &Context,
    ) -> Result<HashMap<usize, RefCell<LayerModel<'a>>>, crate::error::Error> {
        let meshes: HashMap<usize, RefCell<LayerModel<'a>>> = build_test_meshes();

        for value in meshes.values() {
            let trimesh = value.borrow().trimesh.clone();

            value.borrow_mut().model = Some(Gm {
                geometry: Mesh::new(context, &trimesh),
                material: construct_filament_material(),
            });
        }

        for entry in meshes.iter() {
            entry
                .1
                .borrow_mut()
                .model
                .as_mut()
                .unwrap()
                .set_transformation(Mat4::from_translation(vec3(-125.0, 5.0, 125.0)));
        }

        //model.set_transformation(Mat4::from_translation(vec3(0.0, 40.0, 0.0)));

        Ok(meshes)
    }
}

pub fn build_test_meshes<'a>() -> HashMap<usize, RefCell<LayerModel<'a>>> {
    let content = fs::read_to_string("gcode/test2.gcode").unwrap();
    //println!("{}", content);
    let gcode: GCode = content.try_into().unwrap();

    let toolpath = ToolPath::from(gcode);

    let modul_map: HashMap<usize, Vec<PathModul>> = toolpath.into();

    let mut layers: HashMap<usize, RefCell<LayerModel<'a>>> = HashMap::new();

    for entry in modul_map.iter() {
        let layer = LayerModel::empty();
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

    layers
}
