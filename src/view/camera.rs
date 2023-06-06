/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::prelude::*;

use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use bevy::{
    ecs::bundle::Bundle,
    input::{
        mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    },
    time::Time,
    transform::components::Transform,
};

use crate::gui;

#[derive(Default)]
pub struct CameraPlugin {
    pub override_input_system: bool,
}

impl CameraPlugin {
    pub fn _new(override_input_system: bool) -> Self {
        Self {
            override_input_system,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .add_system(control_system)
            .add_event::<CameraControlEvent>();
        
        if !self.override_input_system {
            app.add_system(default_input_map);
        }
    }
}

#[derive(Component, Default)]
pub struct SingleCamera;

#[derive(Bundle)]
pub struct CameraBundle {
    controller: CameraController,
    #[bundle]
    look_transform: LookTransformBundle,
    transform: Transform,
}

impl CameraBundle {
    pub fn new(controller: CameraController, eye: Vec3, target: Vec3, up: Vec3) -> Self {

        let transform = Transform::from_translation(eye).looking_at(target, up);

        Self {
            controller,
            look_transform: LookTransformBundle {
                transform: LookTransform::new(eye, target, up),
                smoother: Smoother::new(controller.smoothing_weight),
            },
            transform,
        }
    }
}


#[derive(Clone, Component, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CameraController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub mouse_translate_sensitivity: Vec2,
    pub mouse_wheel_zoom_sensitivity: f32,
    pub pixels_per_line: f32,
    pub smoothing_weight: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            mouse_rotate_sensitivity: Vec2::splat(0.28),
            mouse_translate_sensitivity: Vec2::splat(0.25),
            mouse_wheel_zoom_sensitivity: 0.1,
            smoothing_weight: 0.4,
            enabled: true,
            pixels_per_line: 53.0,
        }
    }
}

pub enum CameraControlEvent {
    Orbit(Vec2),
    TranslateTarget(Vec2),
    Zoom(f32),
}

pub fn default_input_map(
    mut events: EventWriter<CameraControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    controllers: Query<&CameraController>,
    gui_interface: ResMut<gui::Interface>,
) {

    if gui_interface.is_touch() {
        return;
    }

    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let CameraController {
        mouse_rotate_sensitivity,
        mouse_translate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        events.send(CameraControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
    }

    if mouse_buttons.pressed(MouseButton::Middle) {
        events.send(CameraControlEvent::TranslateTarget(
            mouse_translate_sensitivity * cursor_delta,
        ));
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_reader.iter() {

        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
    }

    events.send(CameraControlEvent::Zoom(scalar));
}

pub fn control_system(
    time: Res<Time>,
    mut events: EventReader<CameraControlEvent>,
    mut cameras: Query<(&CameraController, &mut LookTransform, &Transform)>,
) {

    let (mut transform, scene_transform) =
        if let Some((_, transform, scene_transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
            (transform, scene_transform)
        } else {
            return;
        };

    let mut look_angles = LookAngles::from_vector(-transform.look_direction().unwrap());
    let mut radius_scalar = 1.0;

    let dt = time.delta_seconds();
    for event in events.iter() {
        match event {
            CameraControlEvent::Orbit(delta) => {
                look_angles.add_yaw(dt * -delta.x);
                look_angles.add_pitch(dt * delta.y);
            }
            CameraControlEvent::TranslateTarget(delta) => {
                let right_dir = scene_transform.rotation * -Vec3::X;
                let up_dir = scene_transform.rotation * Vec3::Y;
                transform.target += dt * delta.x * right_dir + dt * delta.y * up_dir;
            }
            CameraControlEvent::Zoom(scalar) => {
                radius_scalar *= scalar;
            }
        }
    }

    look_angles.assert_not_looking_up();

    let new_radius = (radius_scalar * transform.radius())
        .min(1000000.0)
        .max(0.001);
    transform.eye = transform.target + new_radius * look_angles.unit_vector();
}



