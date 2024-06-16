use tokio::task::JoinHandle;
use winit::event::{DeviceEvent, ElementState, WindowEvent};

use crate::{
    geometry::BoundingBox,
    prelude::{Adapter, Error, FrameHandle, SharedMut, WgpuContext},
    GlobalState, RootEvent,
};

mod hitbox;
mod queue;
mod ray;

#[derive(Debug, Clone)]
pub enum PickingEvent {
    Select,
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

impl FrameHandle<'_, RootEvent, (), (GlobalState<RootEvent>, (f32, f32, f32, f32))>
    for PickingAdapter
{
    fn handle_frame(
        &mut self,
        event: &winit::event::Event<RootEvent>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        (state, viewport): (GlobalState<RootEvent>, (f32, f32, f32, f32)),
    ) -> Result<(), Error> {
        let pointer_in_use = state
            .ui_state
            .pointer_in_use
            .inner()
            .load(std::sync::atomic::Ordering::Relaxed);

        if !pointer_in_use {
            println!("PickingAdapter: Pointer not in use");

            match event {
                winit::event::Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. },
                    ..
                } => match button {
                    winit::event::MouseButton::Left => {
                        self.state.is_drag_left = *state == ElementState::Pressed;
                    }
                    winit::event::MouseButton::Right => {
                        self.state.is_drag_left = *state == ElementState::Pressed;
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
                    self.state.is_drag_left = *state == ElementState::Pressed;
                }
                _ => (),
            }
        }

        Ok(())
    }
}

impl<'a>
    Adapter<
        'a,
        RootEvent,
        PickingState,
        (),
        (GlobalState<RootEvent>, (f32, f32, f32, f32)),
        PickingEvent,
    > for PickingAdapter
{
    fn from_context(_wgpu_context: &WgpuContext) -> (PickingState, Self) {
        let state = PickingState {
            hitbox: SharedMut::from_inner(hitbox::HitboxNode::parent_box(BoundingBox::default())),

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
