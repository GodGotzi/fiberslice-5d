/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

mod config;
mod error;
mod gui;
mod math;
mod model;
mod prelude;
mod setup;
mod shortcut;
mod slicer;
mod tests;
mod utils;
mod view;
mod actions;

use std::fs;

use bevy::{prelude::*, render::render_resource::Face};
use gui::UiPlugin;
use model::gcode::GCode;
use prelude::MainPlugin;
use view::{visualization::gcode::create_toolpath, ViewPlugin};

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
        .add_plugins(MainPlugin)
        .add_systems(Startup, spawn_bed)
        .run();
}

fn spawn_bed(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(SceneBundle {
        scene: ass.load("bed.glb#Scene0"),
        transform: Transform::from_scale(Vec3::new(1000.0, 1000.0, 1000.0)),
        ..default()
    });

    let content = fs::read_to_string("gcode/test2.gcode").unwrap();
    let gcode: GCode = content.try_into().unwrap();
    let toolpath = create_toolpath(&gcode);
    let mesh = toolpath.mesh.clone();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                cull_mode: Some(Face::Front),
                reflectance: 0.01,
                metallic: 0.0,
                ..Default::default()
            }),

            transform: Transform::from_translation(Vec3::new(-125.0, 0.3, -125.0)),
            ..Default::default()
        },
    ));
}
