mod fiberslice;

use fiberslice::screen::Screen;

use bevy::{prelude::*, window::PresentMode};
use bevy_egui::{EguiContexts, EguiPlugin};
use smooth_bevy_cameras::{LookTransformPlugin, controllers::orbit::{OrbitCameraPlugin, OrbitCameraBundle, OrbitCameraController}};

fn main() {
    let mut fiberslice = FiberSlice::new();

    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "FiberSlice-3D/5D".into(),
            resolution: (1200., 900.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
        .add_plugin(EguiPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(setup)
        .add_system(move |contexts: EguiContexts| {
            fiberslice.show_ui(contexts)
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 4,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {
    fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }
}

impl FiberSlice {
    fn show_ui(&mut self, mut contexts: EguiContexts) {
        let ctx = contexts.ctx_mut();

        self.screen.ui(ctx);
    }
}