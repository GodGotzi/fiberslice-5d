use bevy::{prelude::*, render::camera::Viewport, window::WindowResized};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmosphereModel, Gradient};

use crate::gui::RawUiData;

use self::camera::SingleCamera;

pub mod camera;
pub mod visualization;

#[allow(dead_code)]
pub enum Orientation {
    Default,
    Diagonal,
    Top,
    Left,
    Right,
    Front,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Preview,
    Prepare,
    ForceAnalytics,
}

/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

pub fn update_camera_viewport(
    windows: Query<&Window>,
    resize_events: EventReader<WindowResized>,
    mut camera: Query<&mut Camera, With<SingleCamera>>,
    data: ResMut<RawUiData>,
) {
    if windows.is_empty() {
        return;
    }

    let result_window = windows.get_single();

    if let Ok(window) = result_window {
        //if !resize_events.is_empty() {
        resize_viewport(window, &mut camera, data);
        //}
    }
}

fn resize_viewport(
    window: &Window,
    camera: &mut Query<&mut Camera, With<SingleCamera>>,
    data: ResMut<RawUiData>,
) {
    let mut camera = camera.single_mut();

    if window.resolution.physical_width() == 0 || window.resolution.physical_height() == 0 {
        return;
    }

    let (viewport_width, viewport_height): (f32, f32) =
        (window.resolution.width(), window.resolution.height());

    //update viewport
    {
        let height = viewport_height
            - ((data.boundary_holder.taskbar().height()
                + data.boundary_holder.modebar().height()
                + data.boundary_holder.menubar().height())
                * window.scale_factor() as f32);

        let viewport = Viewport {
            physical_position: UVec2 {
                x: (data.boundary_holder.toolbar().width() * window.scale_factor() as f32) as u32,
                y: (data.boundary_holder.taskbar().height() * window.scale_factor() as f32) as u32,
            },
            physical_size: UVec2 {
                x: (viewport_width
                    - ((data.boundary_holder.toolbar().width()
                        + data.boundary_holder.settingsbar().width())
                        * window.scale_factor() as f32)) as u32,
                y: height as u32,
            },
            ..default()
        };

        camera.viewport = Some(viewport);
    }
}

pub fn camera_setup(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::PI / 4.0,
                    near: 0.1,
                    far: 100000.0,
                    aspect_ratio: 1.0,
                }),
                ..Default::default()
            },
            AtmosphereCamera::default(),
            SingleCamera,
        ))
        .insert(camera::CameraBundle::new(
            camera::CameraController::default(),
            Vec3::new(250.0, 250.0, 250.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    commands.insert_resource(AtmosphereModel::new(Gradient {
        ground: Color::rgb(0.188, 0.188, 0.188),
        horizon: Color::rgb(0.4, 0.4, 0.4),
        sky: Color::rgb(0.1294, 0.1294, 0.1294),
    }));
}
