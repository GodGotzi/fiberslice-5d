/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use bevy::{math::vec3, prelude::*, render::camera::Viewport};

use bevy_egui::{egui::Pos2, EguiContexts};
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use bevy::{
    ecs::bundle::Bundle,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    time::Time,
    transform::components::Transform,
};

use crate::utils::Contains;

use super::Orientation;

pub trait HandleOrientation {
    fn handle_orientation(&mut self, event: &Orientation);
}

impl HandleOrientation for LookTransform {
    fn handle_orientation(&mut self, event: &Orientation) {
        let position = match event {
            Orientation::Default => vec3(0.00, 250.0, 400.0),
            Orientation::Diagonal => vec3(400.0, 400.0, 400.0),
            Orientation::Top => vec3(0.0, 700.0, 0.1), // FIXME 0.1 is a hack to avoid the camera being inside the model, maybe there is a better way to do this
            Orientation::Left => vec3(-400.0, 0.0, 0.0),
            Orientation::Right => vec3(400.0, 0.0, 0.0),
            Orientation::Front => vec3(0.0, 0.0, 400.0),
        };

        self.eye = position;
    }
}

impl Contains<Pos2> for Viewport {
    fn contains(&self, point: &Pos2) -> bool {
        let x = point.x;
        let y = point.y;

        let x_min = self.physical_position.x as f32;
        let x_max = (self.physical_position.x + self.physical_size.x) as f32;
        let y_min = self.physical_position.y as f32;
        let y_max = (self.physical_position.y + self.physical_size.y) as f32;

        x >= x_min && x <= x_max && y >= y_min && y <= y_max
    }
}

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraControlEvent>()
            .add_systems(Update, control_system)
            .add_systems(Update, handle_events)
            .add_systems(Update, default_input_map);
    }
}

#[derive(Component, Default)]
pub struct SingleCamera;

#[derive(Bundle)]
pub struct CameraBundle {
    controller: CameraController,
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
            mouse_rotate_sensitivity: Vec2::splat(1.0),
            mouse_translate_sensitivity: Vec2::splat(0.25),
            mouse_wheel_zoom_sensitivity: 0.1,
            smoothing_weight: 0.4,
            enabled: true,
            pixels_per_line: 53.0,
        }
    }
}

#[derive(Event)]
pub enum CameraControlEvent {
    Orbit(Vec2),
    TranslateTarget(Vec2),
    Zoom(f32),
}

pub fn default_input_map(
    camera: Query<&mut Camera, With<SingleCamera>>,
    mut events: EventWriter<CameraControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut ui_ctx: EguiContexts,
    mouse_buttons: Res<Input<MouseButton>>,
    controllers: Query<&CameraController>,
) {
    if ui_ctx.ctx_mut().is_using_pointer() {
        return;
    }

    if let Some(mut pos) = ui_ctx.ctx_mut().pointer_latest_pos() {
        if let Ok(camera) = camera.get_single() {
            if let Some(viewport) = camera.viewport.as_ref() {
                pos.x *= ui_ctx.ctx_mut().pixels_per_point();
                pos.y *= ui_ctx.ctx_mut().pixels_per_point();

                if !viewport.contains(&pos) {
                    return;
                }
            }
        }
    }

    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };

    let CameraController {
        mouse_rotate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        mouse_translate_sensitivity,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        events.send(CameraControlEvent::Orbit(
            mouse_rotate_sensitivity * cursor_delta,
        ));
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

fn handle_events(
    mut events: EventReader<Orientation>,
    mut cameras: Query<(&CameraController, &mut LookTransform, &Transform)>,
) {
    let (mut transform, _scene_transform) =
        if let Some((_, transform, scene_transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
            (transform, scene_transform)
        } else {
            return;
        };

    for event in events.iter() {
        transform.handle_orientation(event);
    }
}
