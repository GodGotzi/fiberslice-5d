use std::sync::Arc;
use std::sync::Mutex;

use three_d::Mesh;
use three_d::Srgba;

use crate::application::Application;
use crate::utils::task::VirtualResultTask;

use super::Visualizer;

struct MeshWrapper(Mesh);

impl std::fmt::Debug for MeshWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mesh").finish()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Stroke {
    mesh_wrap: MeshWrapper,
    color: Srgba,
}

#[allow(dead_code)]
pub struct GCodeVisualizer {
    gcode: Option<crate::model::gcode::GCode>,
    result: Option<Arc<Mutex<VirtualResultTask<Vec<Stroke>>>>>,
}

#[allow(dead_code)]
impl GCodeVisualizer {
    pub fn new() -> GCodeVisualizer {
        GCodeVisualizer {
            gcode: None,
            result: None,
        }
    }

    pub fn set_gcode(&mut self, gcode: crate::model::gcode::GCode) {
        self.gcode = Some(gcode);
    }

    pub fn gcode(&self) -> &Option<crate::model::gcode::GCode> {
        &self.gcode
    }
}

#[allow(unused_variables)]
impl Visualizer for GCodeVisualizer {
    fn visualize(&mut self, application: &mut Application) -> Result<(), crate::error::Error> {
        if self.gcode.is_none() {
            return Err(crate::error::Error::FieldMissing("gcode is missing".into()));
        }

        let mut result = VirtualResultTask::<Vec<Stroke>>::new();

        let gcode = self.gcode.as_ref().unwrap().clone();

        result.run(Box::new(move || {
            let mut strokes = Vec::new();

            for instruction in gcode.instructions().iter() {
                /*

                TODO




                */

                //strokes.push(Stroke { mesh_wrap: MeshWrapper(Mesh::new(context, cpu_mesh)), color: () })
            }

            strokes
        }));

        self.result = Some(Arc::new(Mutex::new(result)));

        application
            .task_handler()
            .add_task(self.result.as_ref().unwrap().clone());

        Ok(())
    }

    fn render(
        &self,
        _context: &three_d::WindowedContext,
        application: &mut Application,
    ) -> Result<(), crate::error::Error> {
        todo!()
    }
}
