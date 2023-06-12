/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::{prelude::*, window::WindowResized, render::camera::Viewport};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmosphereModel, Gradient};

use crate::prelude::{Item, AsyncWrapper};

use self::camera::SingleCamera;

pub mod camera;
pub mod orbit;
pub mod mode;

pub enum Orientation {
    Default,
    Top,
    Left, 
    Right,
    Front
}

pub fn update_camera_viewport(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut gui_resize_events: EventReader<Item>,
    mut camera: Query<&mut Camera, With<SingleCamera>>,
    item_wrapper: ResMut<AsyncWrapper>
) {
    if windows.is_empty() {
        return;
    }

    let result_window = windows.get_single();

    let settings_width_packet = item_wrapper.find_packet(Item::SettingsWidth(None)).unwrap();
    let toolbar_width_packet = item_wrapper.find_packet(Item::ToolbarWidth(None)).unwrap();

    if let Ok(window) = result_window {
        for _resize_event in resize_events.iter() {

            if toolbar_width_packet.get_sync().is_some() {

                if let Item::ToolbarWidth(Some(toolbar_width)) = toolbar_width_packet.get_sync().unwrap() {
                    if settings_width_packet.get_sync().is_some() {
                        if let Item::SettingsWidth(Some(settings_width)) = settings_width_packet.get_sync().unwrap() {
                            resize_viewport(window, &mut camera, settings_width, toolbar_width);
                        }
                    }
                }
            
            }

        }
    
        for resize_event in gui_resize_events.iter() {
            if let Item::SettingsWidth(Some(settings_width)) = resize_event {
                if toolbar_width_packet.get_sync().is_some() {
                    if let Item::ToolbarWidth(Some(toolbar_width)) = toolbar_width_packet.get_sync().unwrap() {
                        resize_viewport(window, &mut camera, *settings_width, toolbar_width);
                    } 
                }
            }

            if let Item::ToolbarWidth(Some(toolbar_width)) = resize_event {
                if settings_width_packet.get_sync().is_some() {
                    if let Item::SettingsWidth(Some(settings_width)) = settings_width_packet.get_sync().unwrap() {
                        resize_viewport(window, &mut camera, settings_width, *toolbar_width);
                    }
                }
            }
        }
    }

}

fn resize_viewport(window: &Window, camera: &mut Query<&mut Camera, With<SingleCamera>>, settings_width: f32, toolbar_width: f32) {
    let mut camera = camera.single_mut();

    if window.resolution.physical_width() == 0 || window.resolution.physical_height() == 0  {
        return;
    }

    let new_width = window.resolution.physical_width() as i32 - settings_width as i32 - toolbar_width as i32;

    if new_width < 1 {
        return;
    }

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(toolbar_width as u32, 17),
        physical_size: UVec2::new(
            new_width as u32,
            window.resolution.physical_height() - 51,
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