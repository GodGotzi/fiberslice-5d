/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, window::WindowResized, render::camera::Viewport};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmosphereModel, Gradient};

use crate::fiberslice::gui::GuiResizeEvent;

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
    if windows.is_empty() {
        return;
    }

    let result_window = windows.get_single();

    if let Ok(window) = result_window {
        for _resize_event in resize_events.iter() {
            resize_viewport(window, &mut camera, &view_interface);
        }
    
        for _resize_event in gui_resize_events.iter() {
            resize_viewport(window, &mut camera, &view_interface);
        }
    }

}

fn resize_viewport(window: &Window, camera: &mut Query<&mut Camera, With<SingleCamera>>, view_interface: &ResMut<ViewInterface>) {
    let mut camera = camera.single_mut();

    if window.resolution.physical_width() == 0 || window.resolution.physical_height() == 0  {
        return;
    }

    let new_width = window.resolution.physical_width() as i32 - view_interface.diff_width_side as i32;

    if new_width < 1 {
        return;
    }

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(
            new_width as u32,
            window.resolution.physical_height(),
        ),
        ..default()
    });
}

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 4.0,
            near: 0.1,
            far: 100000.0,
            aspect_ratio: 1.0,
        }),
        ..Default::default()
    }, AtmosphereCamera::default(), SingleCamera::default()))
        .insert(camera::CameraBundle::new(
            camera::CameraController::default(),
            Vec3::new(5.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
    
    commands.insert_resource(AtmosphereModel::new(Gradient {
        ground: Color::rgb(0.188, 0.188, 0.188),
        horizon: Color::rgb(0.4, 0.4, 0.4),
        sky: Color::rgb(0.1294, 0.1294, 0.1294),
    }));
}