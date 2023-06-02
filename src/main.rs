mod fiberslice;
mod view;
mod component;

use component::print_bed::{PrintBed, PrintBedBundle};
use fiberslice::gui::*;
use fiberslice::*;

use bevy_egui::{EguiContexts, EguiPlugin};
use fiberslice::screen::GuiResizeEvent;
use smooth_bevy_cameras::LookTransformPlugin;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution, PrimaryWindow};
use view::camera::CameraPlugin;
use view::orbit::{PossibleOrbitTarget, Orbit, PossibleOrbitBundle};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "FiberSlice-3D/5D".into(),
            resolution: WindowResolution::new(1200., 900.),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
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
        .add_startup_system(view::light_setup)
        .add_startup_system(view::camera_setup)
        .add_startup_system(component_setup)
        .add_startup_system(maximize_window)
        .add_system(view::view_frame)
        .add_system(view::set_camera_viewport)
        .add_system(fiberslice::gui::check_touch)
        .add_system(ui_frame)
        .run();
}

fn component_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn(PrintBedBundle {
        bed: PrintBed,
        orbit_target: PossibleOrbitTarget::new(Orbit::PrintBed),
        material_mesh_bundle: MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                subdivisions: 4,
            })),
            material: materials.add(Color::rgb(123./255., 169./255., 201./255.).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
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

fn maximize_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.set_maximized(true);
}

fn ui_frame(mut contexts: EguiContexts, mut fiberslice: ResMut<FiberSlice>, mut viewinterface: ResMut<view::ViewInterface>, mut events: EventWriter<GuiResizeEvent>) {
    let ctx = contexts.ctx_mut();
    fiberslice.ui_frame(ctx, &mut viewinterface, &mut events);
}