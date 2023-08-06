use three_d::Mesh;
use three_d_asset::{Srgba, Vector3};

use crate::utils::task::VirtualResultTask;

use super::Visualizer;

struct Stroke {
    mesh: Mesh,
    color: Srgba,
    origin: Vector3<f32>,
}

pub struct GCodeVisualizer {
    gcode: Option<crate::model::gcode::GCode>,
    result_stroke_task: Option<VirtualResultTask<Vec<Stroke>>>,
}

impl GCodeVisualizer {
    pub fn new() -> GCodeVisualizer {
        GCodeVisualizer {
            gcode: None,
            result_stroke_task: None,
        }
    }

    pub fn set_gcode(&mut self, gcode: crate::model::gcode::GCode) {
        self.gcode = Some(gcode);
    }

    pub fn gcode(&self) -> &Option<crate::model::gcode::GCode> {
        &self.gcode
    }

    pub fn build_meshes(&self) -> Vec<Mesh> {
        Vec::new()
    }
}

impl Visualizer for GCodeVisualizer {
    fn visualize(&mut self) {}

    fn render(&self, context: &three_d::WindowedContext) {
        todo!()
    }
}
