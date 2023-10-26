use bevy::prelude::Component;

pub mod gcode;
pub mod layer;
pub mod shapes;

#[derive(Component)]
pub struct ToolPath;
