use std::time::Instant;

pub use crate::error::Error;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

use bevy::prelude::*;

use bevy::window::{PrimaryWindow, WindowMode};

#[allow(dead_code)]
pub struct Context {
    now: Instant,
    latest: Instant,
}

impl Context {
    pub fn fps(&self) -> f32 {
        1.0 / (self.now - self.latest).as_secs_f32()
    }
}

pub fn hotkeys_window(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if keyboard_input.pressed(KeyCode::F11) {
        if window.mode == WindowMode::Fullscreen {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::Fullscreen;
        }
    }
}
