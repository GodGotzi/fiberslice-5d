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

use std::{f32::consts::PI, fs, time::Instant};

use bevy::{diagnostic::Diagnostics, prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_atmosphere::prelude::AtmospherePlugin;
use model::gcode::GCode;
use prelude::{AsyncPacket, AsyncWrapper, Item};
use smooth_bevy_cameras::LookTransformPlugin;
use strum::IntoEnumIterator;
use view::{
    camera::CameraPlugin, camera_setup, update_camera_viewport,
    visualization::gcode::create_toolpath,
};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

#[derive(Resource)]
struct FPS {
    now: Instant,
    last: Instant,
}

fn main() {
    let mut list: Vec<AsyncPacket> = Vec::new();

    for item in Item::iter() {
        list.push(AsyncPacket::new(item));
    }

    App::new()
        .add_event::<Item>()
        .insert_resource(AsyncWrapper::new(list))
        .insert_resource(FPS {
            now: Instant::now(),
            last: Instant::now(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LookTransformPlugin)
        .add_plugins(CameraPlugin::default())
        .add_plugins(AtmospherePlugin)
        .add_systems(Startup, camera_setup)
        .add_systems(Startup, spawn_gltf)
        .add_systems(PostStartup, update_window)
        .add_systems(Update, print_fps)
        .add_systems(Update, update_camera_viewport)
        .run();
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("without-textures.glb");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let content = fs::read_to_string("gcode/test2.gcode").unwrap();
    let gcode: GCode = content.try_into().unwrap();
    let toolpath = create_toolpath(&gcode);

    commands.spawn(PbrBundle {
        mesh: meshes.add(toolpath.mesh.clone()),
        // This is the default color, but note that vertex colors are
        // multiplied by the base color, so you'll likely want this to be
        // white if using vertex colors.
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn print_fps(mut fps: ResMut<FPS>) {
    fps.now = Instant::now();

    println!("FPS: {}", 1.0 / (fps.now - fps.last).as_secs_f32());

    fps.last = fps.now;
}

fn update_window(
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();
    primary_window.set_visible(true);

    /*
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
    */
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // example instructions
    commands.spawn(
        TextBundle::from_section(
            "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
            Press 'P' to toggle perspective for line gizmos\n\
            Hold 'Left' or 'Right' to change the line width",
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    gizmos.cuboid(
        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.)),
        Color::BLACK,
    );
    gizmos.rect(
        Vec3::new(time.elapsed_seconds().cos() * 2.5, 1., 0.),
        Quat::from_rotation_y(PI / 2.),
        Vec2::splat(2.),
        Color::GREEN,
    );

    gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, Color::RED);

    for y in [0., 0.5, 1.] {
        gizmos.ray(
            Vec3::new(1., y, 0.),
            Vec3::new(-3., (time.elapsed_seconds() * 3.).sin(), 0.),
            Color::BLUE,
        );
    }

    // Circles have 32 line-segments by default.
    gizmos.circle(Vec3::ZERO, Vec3::Y, 3., Color::BLACK);
    // You may want to increase this for larger circles or spheres.
    gizmos
        .circle(Vec3::ZERO, Vec3::Y, 3.1, Color::NAVY)
        .segments(64);
    gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK)
        .circle_segments(64);
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    if keyboard.just_pressed(KeyCode::D) {
        config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
    }
    if keyboard.just_pressed(KeyCode::P) {
        // Toggle line_perspective
        config.line_perspective ^= true;
        // Increase the line width when line_perspective is on
        config.line_width *= if config.line_perspective { 5. } else { 1. / 5. };
    }

    if keyboard.pressed(KeyCode::Right) {
        config.line_width += 5. * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        config.line_width -= 5. * time.delta_seconds();
    }
}
