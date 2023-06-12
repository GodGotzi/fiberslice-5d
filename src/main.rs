/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

mod view;
mod component;
mod gui;
mod utils;
mod prelude;
mod config;

use bevy_atmosphere::prelude::AtmospherePlugin;
use component::print_bed::{PrintBed, PrintBedBundle};

use bevy_egui::EguiPlugin;
use prelude::{FiberSlice, Item, AsyncWrapper, AsyncPacket};
use smooth_bevy_cameras::LookTransformPlugin;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};

use strum::IntoEnumIterator;
use view::camera::CameraPlugin;
use view::orbit::{PossibleOrbitTarget, Orbit};

fn main() {
    let mut list: Vec<AsyncPacket> = Vec::new();
        
    for item in Item::iter() {
        list.push(AsyncPacket::new(item));
    }

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "FiberSlice-3D/5D".into(),
            resolution: WindowResolution::new(config::default::WINDOW_S.x, config::default::WINDOW_S.y),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: false,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_event::<Item>()
        .insert_resource(AsyncWrapper::new(list))
        .insert_resource(gui::Interface::new())
        .insert_resource(FiberSlice::new())
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugin(EguiPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(CameraPlugin::default())
        .add_plugin(AtmospherePlugin)
        .add_plugin(bevy_stl::StlPlugin)
        .add_startup_system(view::camera_setup)
        .add_startup_system(component_setup)
        .add_startup_system(prelude::maximize_window)
        .add_system(prelude::hotkeys_window)
        .add_system(view::update_camera_viewport)
        .add_system(gui::check_touch)
        .add_system(prelude::ui_frame)
        .run();
}

fn component_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {

    commands.spawn(PrintBedBundle {
        bed: PrintBed,
        orbit_target: PossibleOrbitTarget::new(Orbit::PrintBed),
        material_mesh_bundle: MaterialMeshBundle {
            mesh: asset_server.load("stifhalterung.stl"),
            material: materials.add(
                Color::rgb(188./255., 230./255., 124./255.).into()
            ),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..Default::default()
        },
    });

/*
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 4,
        })),
        material: materials.add(Color::rgb(123./255., 169./255., 201./255.).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(PrintBed)
    .insert(PossibleOrbitTarget::new(Orbit::PrintBed));
*/
}