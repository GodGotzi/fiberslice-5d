use tokio::task::JoinHandle;
use winit::event::{DeviceEvent, ElementState, WindowEvent};

use crate::{
    camera::CameraResult,
    geometry::BoundingHitbox,
    prelude::{Adapter, Error, FrameHandle, SharedMut, WgpuContext},
    render::{
        buffer::BufferLocation,
        model::{MeshHandle, Model},
        vertex::Vertex,
    },
    GlobalState, RootEvent,
};

pub mod hitbox;
mod queue;
pub mod ray;

#[derive(Debug, Clone)]
pub enum PickingEvent {
    AddInteractiveMesh(MeshHandle),
}

pub trait Pickable: std::fmt::Debug + Send + Sync {
    fn hover(&self, state: GlobalState<RootEvent>);
    fn select(&self, state: GlobalState<RootEvent>);
}

#[derive(Debug, Clone)]
pub struct PickingState {
    hitbox: SharedMut<hitbox::HitboxNode>,

    is_drag_left: bool,
    is_drag_right: bool,
}

pub struct PickingAdapter {
    handles: Vec<JoinHandle<()>>,

    state: PickingState,
}

impl FrameHandle<'_, RootEvent, (), (GlobalState<RootEvent>, &CameraResult)> for PickingAdapter {
    fn handle_frame(
        &mut self,
        event: &winit::event::Event<RootEvent>,
        _start_time: std::time::Instant,
        _wgpu_context: &WgpuContext,
        (global_state, camera_result): (GlobalState<RootEvent>, &CameraResult),
    ) -> Result<(), Error> {
        let pointer_in_use = global_state
            .ui_state
            .pointer_in_use
            .load(std::sync::atomic::Ordering::Relaxed);

        if !pointer_in_use {
            let CameraResult {
                view,
                proj,
                viewport,
                eye,
            } = *camera_result;

            match event {
                winit::event::Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. },
                    ..
                } => match button {
                    winit::event::MouseButton::Left => {
                        self.state.is_drag_left = *state == ElementState::Pressed;
                    }
                    winit::event::MouseButton::Right => {
                        if let Some((x, y)) = global_state.ctx.mouse_position {
                            let ray = ray::Ray::from_view(viewport, (x, y), view, proj, eye);

                            self.state.hitbox.read_with_fn(|root| {
                                println!("PickingAdapter: Checking Hit");

                                let hit = root.check_hit(&ray);

                                if let Some(handle) = hit {
                                    println!("PickingAdapter: Hit: {:?}", handle);
                                }
                            });
                        }

                        self.state.is_drag_right = *state == ElementState::Pressed;
                    }
                    _ => (),
                },
                winit::event::Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    if self.state.is_drag_left {
                        println!("PickingAdapter: Dragging Left Click");
                    }

                    if self.state.is_drag_right {
                        println!("PickingAdapter: Dragging Right Click");
                    }
                }
                winit::event::Event::UserEvent(RootEvent::PickingEvent(
                    PickingEvent::AddInteractiveMesh(handle),
                )) => {
                    self.state.hitbox.write_with_fn(|root| {
                        let hitbox = handle.clone().into();

                        root.add_hitbox(hitbox);
                    });
                    println!("PickingAdapter: Adding Interactive Mesh");
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
                    self.state.is_drag_left = *state == ElementState::Pressed;
                }
                winit::event::MouseButton::Right => {
                    self.state.is_drag_right = *state == ElementState::Pressed;
                }
                _ => (),
            }
        }

        Ok(())
    }
}

impl<'a>
    Adapter<'a, RootEvent, PickingState, (), (GlobalState<RootEvent>, &CameraResult), PickingEvent>
    for PickingAdapter
{
    fn from_context(_wgpu_context: &WgpuContext) -> (PickingState, Self) {
        let state = PickingState {
            hitbox: SharedMut::from_inner(hitbox::HitboxNode::parent_box(SharedMut::from_inner(
                Box::<BoundingHitbox>::default(),
            ))),

            is_drag_left: false,
            is_drag_right: false,
        };

        (
            state.clone(),
            PickingAdapter {
                handles: vec![],
                state,
            },
        )
    }

    fn get_adapter_description(&self) -> String {
        "PickingAdapter".to_string()
    }
}
