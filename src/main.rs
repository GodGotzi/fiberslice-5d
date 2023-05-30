mod fiberslice;
mod view;
mod component;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use component::EguiData;
use fiberslice::screen::Screen;

use bevy_egui::{EguiContexts, EguiPlugin};
use image::flat::View;
use smooth_bevy_cameras::LookTransformPlugin;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use view::{camera::*, ViewInterface};

/*
fn check_cursor_on_egui_element(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
) {
    if let Some(cursor_position) = egui_context.ctx().input().mouse().pos {
        for (_, window) in windows.iter() {
            let window_size = window.physical_size();
            if cursor_position.x >= 0.0
                && cursor_position.x <= window_size.width as f32
                && cursor_position.y >= 0.0
                && cursor_position.y <= window_size.height as f32
            {
                // Cursor is within the window boundaries, you can perform further checks on specific `egui` elements here
                println!("Cursor is on an egui element!");
            }
        }
    }
}

 */

fn main() {
        
    //let mut view_interface = ViewInterface::new(camera3d);

    let mut fiberslice = FiberSlice::new();
    let mut egui_data = EguiData::new();

    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
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
    }))
        .add_plugin(EguiPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(CameraPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(maximize_window)
        .add_system(move |contexts: EguiContexts, windows: Query<&mut Window>| {
            egui_data.check_touch(contexts, windows)
        })
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
        material: materials.add(Color::rgb(
            rgb_to_one_zero(123.), 
        rgb_to_one_zero(169.), 
        rgb_to_one_zero(201.)).into()),
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

    commands.spawn(Camera3dBundle::default())
        .insert(CameraBundle::new(
            CameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

fn maximize_window(
    // we have to use `NonSend` here
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.set_maximized(true);
} 

fn rgb_to_one_zero(rgb: f32) -> f32 {
    rgb/255.0
}

struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {
    fn new(/*_view_interface: ViewInterface*/) -> Self {
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