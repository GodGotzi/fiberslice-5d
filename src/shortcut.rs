use bevy::{input::keyboard, prelude::*};

use crate::view::camera::CameraControlEvent;

pub struct ShortcutPlugin;

impl Plugin for ShortcutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, window::hotkeys_window)
            .add_systems(Update, handle_shortcut);
    }
}

pub fn handle_shortcut(
    //mut shortcut_writer: EventWriter<crate::shortcut::Shortcut>,
    mut camera_events: EventWriter<CameraControlEvent>,
    keyboard: Res<Input<keyboard::KeyCode>>,
) {
    if (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight))
        && keyboard.pressed(KeyCode::R)
    {
        camera_events.send(CameraControlEvent::TargetUpdate(Vec3::ZERO));
    }
}

mod window {
    use bevy::{
        prelude::*,
        window::{PrimaryWindow, WindowMode},
    };

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
}
