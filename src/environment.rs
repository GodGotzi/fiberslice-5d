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
        let (distance, yaw, pitch) = match orientation {
            Orientation::Default => (100.0, 1.5, 1.25),
            Orientation::Diagonal => (100.0, PI / 4.0, PI / 4.0),
            Orientation::Top => (100.0, 0.0, PI / 2.0),
            Orientation::Left => (100.0, PI / 2.0, 0.0),
            Orientation::Right => (100.0, -PI / 2.0, 0.0),
            Orientation::Front => (100.0, 0.0, 0.0),
        };

        self.set_distance(distance);
        self.set_yaw(yaw);
        self.set_pitch(pitch);
    }
}
