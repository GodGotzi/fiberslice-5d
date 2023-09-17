use crate::application::Application;

pub mod force;
pub mod gcode;

pub trait Visualizer<O> {
    fn visualize(&mut self, application: &mut Application) -> Result<(), crate::error::Error>;

    fn try_collect_objects(
        &self,
        context: &three_d::WindowedContext,
    ) -> Result<Vec<O>, crate::error::Error>;
}

#[allow(dead_code)]
pub struct VisualizerContext {
    pub gcode: gcode::GCodeVisualizer,
    pub force: force::ForceVisualizer,
}

impl Default for VisualizerContext {
    fn default() -> Self {
        Self {
            gcode: gcode::GCodeVisualizer::new(),
            force: force::ForceVisualizer::new(),
        }
    }
}

#[allow(dead_code)]
impl VisualizerContext {
    pub fn new() -> VisualizerContext {
        Self::default()
    }
}
