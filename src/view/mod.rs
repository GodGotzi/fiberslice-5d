/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use bevy::{prelude::*, render::camera::Viewport};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmosphereModel, AtmospherePlugin, Gradient};
use smooth_bevy_cameras::LookTransformPlugin;
use strum_macros::{EnumCount, EnumIter};

use crate::ui::data::RawUiData;

use self::camera::{CameraPlugin, SingleCamera};

pub mod camera;
pub mod visualization;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Orientation>()
            .add_plugins(LookTransformPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(AtmospherePlugin)
            .add_systems(Startup, environment_setup)
            .add_systems(Update, update_viewport);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Event, EnumCount, EnumIter)] //maybe performance bit worse
pub enum Orientation {
    Default,
    Diagonal,
    Top,
    Left,
    Right,
    Front,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Event)]
pub enum Mode {
    Preview,
    Prepare,
    ForceAnalytics,
}

pub fn update_viewport(
    windows: Query<&Window>,
    //resize_events: EventReader<WindowResized>,
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

    let (viewport_width, viewport_height): (u32, u32) = (
        window.resolution.physical_width(),
        window.resolution.physical_height(),
    );

    {
        let height = viewport_height
            - ((data.holder.taskbar.boundary().height()
                + data.holder.modebar.boundary().height()
                + data.holder.menubar.boundary().height())
                * window.scale_factor() as f32) as u32
            + 4;

        let viewport = Viewport {
            physical_position: UVec2 {
                x: (data.holder.toolbar.boundary().width() * window.scale_factor() as f32).max(2.0)
                    as u32
                    - 2,
                y: (data.holder.menubar.boundary().height() * window.scale_factor() as f32).max(2.0)
                    as u32
                    - 2,
            },
            physical_size: UVec2 {
                x: (viewport_width
                    - ((data.holder.toolbar.boundary().width()
                        + data.holder.settingsbar.boundary().width())
                        * window.scale_factor() as f32) as u32)
                    + 4,
                y: height,
            },
            ..default()
        };

        camera.viewport = Some(viewport);
    }
}

pub fn environment_setup(mut commands: Commands) {
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

    commands.insert_resource(AmbientLight {
        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        brightness: 1.0,
    });

    commands.insert_resource(AtmosphereModel::new(Gradient {
        ground: Color::rgba(0.8, 0.8, 0.8, 1.0),
        horizon: Color::rgba(0.5, 0.5, 0.5, 1.0),
        sky: Color::rgba(0.8, 0.8, 0.8, 1.0),
    }));
}
