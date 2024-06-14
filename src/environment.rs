use std::f32::consts::PI;

pub mod camera_controller;
pub mod view;

use crate::render::camera::OrbitCamera;

use view::Orientation;

pub trait HandleOrientation {
    fn handle_orientation(&mut self, orientation: Orientation);
}

impl HandleOrientation for OrbitCamera {
    fn handle_orientation(&mut self, orientation: Orientation) {
        let (yaw, pitch) = match orientation {
            Orientation::Default => (PI / 8.0, PI / 4.0),
            Orientation::Diagonal => (PI / 8.0, PI / 4.0),
            Orientation::Top => (0.0, PI / 2.0),
            Orientation::Left => (PI / 2.0, 0.0),
            Orientation::Right => (-PI / 2.0, 0.0),
            Orientation::Front => (0.0, 0.0),
        };

        self.set_yaw(yaw);
        self.set_pitch(pitch);
    }
}
