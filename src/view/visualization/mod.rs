pub mod force;
pub mod model;

pub trait Visualizer {
    fn visualize(&mut self);

    fn render(&self, context: &three_d::WindowedContext);
}

pub struct VisualizerContext {
    gcode: model::GCodeVisualizer,
    force: force::ForceVisualizer,
}

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
