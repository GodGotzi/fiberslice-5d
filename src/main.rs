/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

mod application;
mod config;
mod error;
mod gui;
mod import;
mod math;
mod model;
mod prelude;
mod setup;
mod slicer;
mod tests;
mod utils;
mod view;
mod window;

use crate::window::setup_window;
use window::update_window;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "FiberSlice-5D".into(),
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_window)
        .add_systems(Update, update_window)
        .run();
}
