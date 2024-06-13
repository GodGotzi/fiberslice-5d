use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, ElementState, Event, MouseScrollDelta, WindowEvent},
    window::Window,
};

use crate::render::camera::OrbitCamera;

pub struct CameraController {
    pub rotate_speed: f32,
    pub zoom_speed: f32,
    is_drag_rotate: bool,
    is_drag_move: bool,
}

impl CameraController {
    pub fn new(rotate_speed: f32, zoom_speed: f32) -> Self {
        Self {
            rotate_speed,
            zoom_speed,
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
                        camera.target.x -= delta.0 as f32 * 0.01 * camera.yaw.cos();
                        camera.eye.x -= delta.0 as f32 * 0.01 * camera.yaw.cos();

                        camera.target.z -= delta.1 as f32 * 0.01 * camera.yaw.sin();
                        camera.eye.z -= delta.1 as f32 * 0.01 * camera.yaw.sin();

                        // camera.target.y += delta.1 as f32 * self.rotate_speed;
                        // camera.eye.y += delta.1 as f32 * self.rotate_speed;
                        // camera.target.z += delta.1 as f32;

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
