use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, window::WindowResized, render::camera::Viewport};

use crate::fiberslice::screen::GuiResizeEvent;

use self::camera::SingleCamera;

pub mod camera;
pub mod orbit;

#[derive(Resource)]
pub struct ViewInterface {
    new_view_color: Option<Color>,
    pub diff_width_side: u32,
}

impl ViewInterface {
    pub fn new() -> Self {
        Self {
            new_view_color: None,
            diff_width_side: 150
        }
    }

    pub fn change_view_color(&mut self, r: f32, g: f32, b: f32) {
        self.new_view_color = Some(Color::rgb(r, g, b));
    }

    pub fn need_view_color_changed(&mut self) -> Option<Color> {
        self.new_view_color
    }

    pub fn reset_need_view_color_changed(&mut self) {
        self.new_view_color = None;
    }
}

pub fn view_frame(mut camera_query: Query<&mut Camera3d>, mut view_interface: ResMut<ViewInterface>) {
    if let Some(color) = view_interface.need_view_color_changed() {
        view_interface.reset_need_view_color_changed();

        camera_query.for_each_mut(|mut camera| {
            camera.clear_color = ClearColorConfig::Custom(color)
        });
    }
}

pub fn set_camera_viewport(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut gui_resize_events: EventReader<GuiResizeEvent>,
    mut camera: Query<&mut Camera, With<SingleCamera>>,
    view_interface: ResMut<ViewInterface>
) {
    let window = windows.single();
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for _resize_event in resize_events.iter() {
        let mut camera = camera.single_mut();
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() - view_interface.diff_width_side,
                window.resolution.physical_height(),
            ),
            ..default()
        });
    }

    for _resize_event in gui_resize_events.iter() {
        let mut camera = camera.single_mut();
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() - view_interface.diff_width_side,
                window.resolution.physical_height(),
            ),
            ..default()
        });
    }
    
}

pub fn light_setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), SingleCamera::default()))
        .insert(camera::CameraBundle::new(
            camera::CameraController::default(),
            Vec3::new(5.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}