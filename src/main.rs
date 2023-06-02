mod fiberslice;
mod view;
mod component;
mod window;

use bevy_atmosphere::prelude::AtmospherePlugin;
use component::print_bed::{PrintBed, PrintBedBundle};
use fiberslice::gui::*;
use fiberslice::*;

use bevy_egui::EguiPlugin;
use fiberslice::screen::GuiResizeEvent;
use smooth_bevy_cameras::LookTransformPlugin;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use view::camera::CameraPlugin;
use view::orbit::{PossibleOrbitTarget, Orbit};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "FiberSlice-3D/5D".into(),
            resolution: WindowResolution::new(1200., 900.),
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
        .add_event::<GuiResizeEvent>()
        .insert_resource(view::ViewInterface::new())
        .insert_resource(GuiInterface::new())
        .insert_resource(FiberSlice::new())
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugin(EguiPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(CameraPlugin::default())
        .add_plugin(AtmospherePlugin)
        .add_plugin(bevy_stl::StlPlugin)
        .add_startup_system(view::camera_setup)
        .add_startup_system(component_setup)
        .add_startup_system(window::maximize_window)
        .add_system(window::hotkeys_window)
        .add_system(view::view_frame)
        .add_system(view::set_camera_viewport)
        .add_system(fiberslice::gui::check_touch)
        .add_system(fiberslice::gui::ui_frame)
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
            mesh: asset_server.load("print_bed.stl"),
            material: materials.add(Color::rgb(123./255., 169./255., 201./255.).into()),
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