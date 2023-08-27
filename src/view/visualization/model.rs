use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use three_d::*;
use three_d_asset::TriMesh;

use crate::application::Application;
use crate::model::gcode::toolpath::PathModulMesh;
use crate::model::gcode::toolpath::ToolPath;
use crate::model::gcode::GCode;
use crate::utils::debug::DebugWrapper;
use crate::utils::task::TaskWithResult;

use super::Visualizer;

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

impl Visualizer for GCodeVisualizer {
    fn visualize(&mut self, application: &mut Application) -> Result<(), crate::error::Error> {
        if self.gcode.is_none() {
            return Err(crate::error::Error::FieldMissing("gcode is missing".into()));
        }

        let mut result = TaskWithResult::<Vec<Layer>>::new();

        let gcode = self.gcode.as_ref().unwrap().clone();

        result.run(Box::new(move || {
            let layers = Vec::new();

            for _instruction in gcode.instructions().iter() {
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

    fn try_collect_objects(
        &self,
        context: &three_d::WindowedContext,
    ) -> Result<Vec<Box<dyn Object>>, crate::error::Error> {
        let mut objects: Vec<Box<dyn Object>> = Vec::new();

        let meshes = build_test_meshes();

        for mesh in meshes {
            let mut model: Gm<Mesh, PhysicalMaterial> = Gm::new(
                Mesh::new(context, mesh.mesh()),
                PhysicalMaterial::new(
                    context,
                    &CpuMaterial {
                        albedo: *mesh.color(),
                        ..Default::default()
                    },
                ),
            );

            model.set_transformation(
                Mat4::from_translation(vec3(-100.0, 5.0, 50.0))
                    .concat(&Mat4::from_angle_y(degrees(45.0))),
            );

            objects.push(Box::new(model));
        }

        //model.set_transformation(Mat4::from_translation(vec3(0.0, 40.0, 0.0)));

        Ok(objects)
    }
}

pub fn build_test_meshes() -> Vec<PathModulMesh> {
    let content = fs::read_to_string("gcode/test.gcode").unwrap();
    //println!("{}", content);
    let gcode: GCode = content.try_into().unwrap();

    let toolpath = ToolPath::from(gcode);

    let meshes: Vec<PathModulMesh> = std::convert::Into::<Vec<PathModulMesh>>::into(toolpath);
    meshes
}
