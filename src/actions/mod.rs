use bevy::prelude::*;

use crate::view::camera::CameraControlEvent;

pub mod file;

#[derive(Default)]
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraControlEvent>()
            .add_systems(Update, file::handle_tasks);
    }
}
