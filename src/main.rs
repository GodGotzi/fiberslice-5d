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
mod slicer;
mod tests;
mod utils;
mod view;

use std::{fs, time::Instant};

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_atmosphere::prelude::AtmospherePlugin;
use bevy_egui::EguiPlugin;
use gui::{ui_frame, RawUiData, Screen};
use model::gcode::GCode;
use prelude::hotkeys_window;
use smooth_bevy_cameras::LookTransformPlugin;
use view::{
    camera::CameraPlugin, camera_setup, update_camera_viewport,
    visualization::gcode::create_toolpath,
};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use winit::window::Icon;

#[derive(Resource)]
struct FPS {
    now: Instant,
    last: Instant,
}

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
        .insert_resource(Screen::new())
        .insert_resource(RawUiData::new(gui::Theme::Light, view::Mode::Prepare))
        .insert_resource(FPS {
            now: Instant::now(),
            last: Instant::now(),
        })
        .add_plugins(DefaultPlugins.set(plugin))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(LookTransformPlugin)
        .add_plugins(CameraPlugin::default())
        .add_plugins(AtmospherePlugin)
        .add_systems(Startup, camera_setup)
        .add_systems(Startup, spawn_bed)
        .add_systems(PostStartup, init_window)
        .add_systems(Update, print_fps)
        .add_systems(Update, update_camera_viewport)
        .add_systems(Update, ui_frame)
        .add_systems(Update, hotkeys_window)
        .run();
}

fn spawn_bed(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(SceneBundle {
        scene: ass.load("bed_new.glb#Scene0"),
        transform: Transform::from_scale(Vec3::new(1000.0, 1000.0, 1000.0)),
        ..default()
    });

    let content = fs::read_to_string("gcode/test2.gcode").unwrap();
    let gcode: GCode = content.try_into().unwrap();
    let toolpath = create_toolpath(&gcode);

    commands.spawn(PbrBundle {
        mesh: meshes.add(toolpath.mesh.clone()),
        // This is the default color, but note that vertex colors are
        // multiplied by the base color, so you'll likely want this to be
        // white if using vertex colors.
        material: materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(-125.0, 0.2, -125.0)),
        ..Default::default()
    });
}

fn print_fps(mut fps: ResMut<FPS>) {
    fps.now = Instant::now();

    println!("FPS: {}", 1.0 / (fps.now - fps.last).as_secs_f32());

    fps.last = fps.now;
}

fn init_window(
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();
    primary_window.set_visible(true);

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/main_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}
