use bevy::prelude::Component;

pub mod gcode;
pub mod layer;

#[derive(Component)]
pub struct ToolPath;