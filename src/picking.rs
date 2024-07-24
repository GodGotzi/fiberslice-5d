use hitbox::HitboxNode;
use tokio::task::JoinHandle;
use winit::event::{DeviceEvent, ElementState, WindowEvent};

use crate::{
    camera::CameraResult,
    prelude::{
        create_event_bundle, Adapter, AdapterCreation, Error, EventReader, FrameHandle, SharedMut,
        WgpuContext,
    },
    GlobalState, RootEvent,
};

pub mod hitbox;
pub mod interactive;
mod queue;
pub mod ray;

#[derive(Debug)]
pub enum PickingEvent {
    Pick,
    AddHitbox(HitboxNode),
}

#[derive(Debug, Clone)]
pub struct PickingState {
    hitbox: SharedMut<hitbox::HitboxNode>,
    is_drag_left: bool,
    is_drag_right: bool,
}

impl PickingState {
    pub fn add_hitbox(&self, hitbox: hitbox::HitboxNode) {
        self.hitbox.write_with_fn(|root| {
            root.add_node(hitbox);
        });
    }
}

pub struct PickingAdapter {
    handles: Vec<JoinHandle<()>>,
    state: PickingState,

    event_reader: EventReader<PickingEvent>,
}

impl FrameHandle<'_, RootEvent, (), (GlobalState<RootEvent>, &CameraResult)> for PickingAdapter {
    fn handle_frame(
        &mut self,
        event: &winit::event::Event<RootEvent>,
        _start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
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

                                let hit = root.check_hit(&ray, wgpu_context);

                                if let Some(handle) = hit {
                                    println!("PickingAdapter: Hit");
                                    handle.read().picked(&global_state, wgpu_context);
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
                        if let Some((x, y)) = global_state.ctx.mouse_position {
                            let ray = ray::Ray::from_view(viewport, (x, y), view, proj, eye);

                            self.state.hitbox.read_with_fn(|root| {
                                println!("PickingAdapter: Checking Hit");

                                let hit = root.check_hit(&ray, wgpu_context);

                                if let Some(handle) = hit {
                                    println!("PickingAdapter: Hit");
                                    handle.read().picked(&global_state, wgpu_context);
                                }
                            });
                        }
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
    fn create(_wgpu_context: &WgpuContext) -> AdapterCreation<PickingState, PickingEvent, Self> {
        let state = PickingState {
            hitbox: SharedMut::from_inner(hitbox::HitboxNode::root()),

            is_drag_left: false,
            is_drag_right: false,
        };

        let (reader, writer) = create_event_bundle::<PickingEvent>();

        (
            state.clone(),
            writer,
            PickingAdapter {
                handles: vec![],
                state,
                event_reader: reader,
            },
        )
    }

    fn get_adapter_description(&self) -> String {
        "PickingAdapter".to_string()
    }

    fn get_reader(&self) -> crate::prelude::EventReader<PickingEvent> {
        self.event_reader.clone()
    }

    fn handle_event(
        &mut self,
        _wgpu_context: &WgpuContext,
        _global_state: &GlobalState<RootEvent>,
        event: PickingEvent,
    ) {
        match event {
            PickingEvent::AddHitbox(box_) => {
                self.state.hitbox.write_with_fn(|root| {
                    root.add_node(box_.clone());
                });
                println!("PickingAdapter: Adding Interactive Mesh");
            }
            PickingEvent::Pick => {}
        }
    }
}
