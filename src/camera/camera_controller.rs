use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, ElementState, Event, MouseScrollDelta, WindowEvent},
    window::Window,
};

use super::OrbitCamera;

#[derive(Debug)]
pub struct CameraController {
    pub rotate_speed: f32,
    pub zoom_speed: f32,
    pub move_speed: f32,

    is_drag_rotate: bool,
    is_drag_move: bool,
}

impl CameraController {
    pub fn new(rotate_speed: f32, zoom_speed: f32, move_speed: f32) -> Self {
        Self {
            rotate_speed,
            zoom_speed,
            move_speed,
            is_drag_rotate: false,
            is_drag_move: false,
        }
    }

    pub fn process_events<T>(
        &mut self,
        event: &Event<T>,
        window: &Window,
        camera: &mut OrbitCamera,
        pointer_in_use: bool,
    ) {
        if !pointer_in_use {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::MouseWheel { delta, .. } => {
                        let scroll_amount = -match delta {
                            // A mouse line is about 1 px.
                            MouseScrollDelta::LineDelta(_, scroll) => scroll * 1.0,
                            MouseScrollDelta::PixelDelta(PhysicalPosition {
                                y: scroll, ..
                            }) => *scroll as f32,
                        };
                        camera.add_distance(scroll_amount * self.zoom_speed);
                        window.request_redraw();
                    }
                    WindowEvent::MouseInput { button, state, .. } => match button {
                        winit::event::MouseButton::Left => {
                            self.is_drag_rotate = *state == ElementState::Pressed;
                        }
                        winit::event::MouseButton::Middle => {
                            self.is_drag_move = *state == ElementState::Pressed;
                        }
                        _ => (),
                    },
                    _ => (),
                },
                winit::event::Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    if self.is_drag_rotate {
                        camera.add_yaw(delta.0 as f32 * self.rotate_speed);
                        camera.add_pitch(delta.1 as f32 * self.rotate_speed);
                        window.request_redraw();
                    } else if self.is_drag_move {
                        let direction = (camera.target - camera.eye).normalize();
                        let right = direction.cross(camera.up).normalize();
                        let up = right.cross(direction).normalize();

                        let move_amount = right * delta.0 as f32 + up * delta.1 as f32;

                        camera.eye += move_amount * self.move_speed;
                        camera.target += move_amount * self.move_speed;

                        window.request_redraw();
                    }
                }
                _ => (),
            }
        } else if let &winit::event::Event::WindowEvent {
            event: WindowEvent::MouseInput { button, state, .. },
            ..
        } = &event
        {
            match button {
                winit::event::MouseButton::Left => {
                    self.is_drag_rotate = *state == ElementState::Pressed;
                }
                winit::event::MouseButton::Middle => {
                    self.is_drag_move = *state == ElementState::Pressed;
                }
                _ => (),
            }
        }
    }
}
