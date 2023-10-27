use bevy::prelude::Component;

pub mod gcode;
pub mod mesh;
pub mod shapes;

#[derive(Component)]
pub struct ToolPath;
