use bevy::prelude::{App, Plugin, Update};

pub struct ShortcutPlugin;

impl Plugin for ShortcutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, window::hotkeys_window);
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
