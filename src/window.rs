/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::{prelude::*, window::{PrimaryWindow, WindowMode}};

pub fn maximize_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.set_maximized(true);
}

pub fn hotkeys_window(mut windows: Query<&mut Window, With<PrimaryWindow>>, keyboard_input: Res<Input<KeyCode>>) {

    let mut window = windows.single_mut();

    if keyboard_input.pressed(KeyCode::F11) {
        if window.mode == WindowMode::Fullscreen {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::Fullscreen;
        }
    }

}