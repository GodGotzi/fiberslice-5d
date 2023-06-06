/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::{prelude::*, window::WindowResized, render::camera::Viewport};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmosphereModel, Gradient};

use crate::prelude::{Item, AsyncWrapper, ItemType};

use self::camera::SingleCamera;

pub mod camera;
pub mod orbit;
pub mod visualization;

pub fn update_camera_viewport(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut gui_resize_events: EventReader<Item>,
    mut camera: Query<&mut Camera, With<SingleCamera>>,
    item_wrapper: ResMut<AsyncWrapper<ItemType, Item>>
) {
    if windows.is_empty() {
        return;
    }

    let result_window = windows.get_single();

    if let Ok(window) = result_window {
        for _resize_event in resize_events.iter() {
            let width_packet = item_wrapper.gui_events.get(&ItemType::SideWidth).unwrap();

            if width_packet.get_sync().is_some() {
                if let Item::SideWidth(width) = width_packet.get_sync().unwrap() {
                    resize_viewport(window, &mut camera, width);
                } else {
                    panic!("ItemType isn't what I suspected to be!");
                }
            }
        }
    
        for resize_event in gui_resize_events.iter() {
            if let Item::SideWidth(width) = resize_event {
                resize_viewport(window, &mut camera, *width);
            }
        }
    }

}

fn resize_viewport(window: &Window, camera: &mut Query<&mut Camera, With<SingleCamera>>, width: f32) {
    let mut camera = camera.single_mut();

    if window.resolution.physical_width() == 0 || window.resolution.physical_height() == 0  {
        return;
    }

    let new_width = window.resolution.physical_width() as i32 - width as i32;

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