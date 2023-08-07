use three_d::Object;

use crate::application::Application;

pub mod force;
pub mod model;

pub trait Visualizer {
    fn visualize(&mut self, application: &mut Application) -> Result<(), crate::error::Error>;

    fn try_collect_objects(
        &self,
        context: &three_d::WindowedContext,
    ) -> Result<Vec<Box<dyn Object>>, crate::error::Error>;
}

#[allow(dead_code)]
pub struct VisualizerContext {
    gcode: model::GCodeVisualizer,
    force: force::ForceVisualizer,
}

#[allow(dead_code)]
impl VisualizerContext {
    pub fn new() -> VisualizerContext {
        VisualizerContext {
            gcode: model::GCodeVisualizer::new(),
            force: force::ForceVisualizer::new(),
        }
    }

    pub fn gcode(&mut self) -> &mut model::GCodeVisualizer {
        &mut self.gcode
    }

    pub fn force(&mut self) -> &mut force::ForceVisualizer {
        &mut self.force
    }
}
