use bevy::{prelude::{Camera3d, Color}, core_pipeline::clear_color::ClearColorConfig};

pub mod camera;

pub struct ViewInterface<'a> {
    camera3d: &'a mut Camera3d,
}

impl <'a> ViewInterface<'a> {
    pub fn new(camera3d: &'a mut Camera3d) -> Self {
        Self {
            camera3d
        }
    }

    pub fn set_view_color(&self, r: f32, g: f32, b: f32) {
        
    }
}