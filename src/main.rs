/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

mod actions;
mod api;
mod config;
mod error;
mod math;
mod model;
mod prelude;
mod settings;
mod shortcut;
mod slicer;
mod tests;
mod ui;
mod view;

use actions::ActionPlugin;
use bevy::prelude::*;
use prelude::MainPlugin;
use settings::SettingsPlugin;
use shortcut::ShortcutPlugin;
use ui::UiPlugin;
use view::ViewPlugin;

fn main() {
    let plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Fiberslice-5D".into(),
            resolution: config::default::WINDOW_S.into(),
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(plugin))
        .add_plugins(UiPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(ShortcutPlugin)
        .add_plugins(ActionPlugin)
        .add_plugins(MainPlugin)
        //.add_systems(Startup, spawn_bed)
        .run();
}
