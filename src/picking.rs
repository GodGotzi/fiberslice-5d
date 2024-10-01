use glam::vec3;
use rether::picking::interact::InteractiveModel;
use rether::{model::TranslateModel, picking::interact::ClickEvent};
use tokio::task::JoinHandle;
use winit::event::{DeviceEvent, ElementState, WindowEvent};

use crate::{
    camera::CameraResult,
    prelude::{
        create_event_bundle, Adapter, AdapterCreation, Error, EventReader, FrameHandle, WgpuContext,
    },
    GlobalState, RootEvent,
};

#[derive(Debug)]
pub enum PickingEvent {
    Pick,
}

#[derive(Debug, Clone)]
pub struct PickingState {
    is_drag_left: bool,
    is_drag_right: bool,
}

pub struct PickingAdapter {
    handles: Vec<JoinHandle<()>>,
    state: PickingState,

    camera_result: Option<CameraResult>,
    event_reader: EventReader<PickingEvent>,
}

impl FrameHandle<'_, RootEvent, (), &CameraResult> for PickingAdapter {
    fn handle_frame(
        &mut self,
        _wgpu_context: &WgpuContext,
        _global_state: GlobalState<RootEvent>,
        camera_result: &CameraResult,
    ) -> Result<(), Error> {
        self.camera_result = Some(camera_result.clone());

        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: &WindowEvent,
        _id: winit::window::WindowId,
        wgpu_context: &WgpuContext,
        global_state: GlobalState<RootEvent>,
    ) {
        let pointer_in_use = global_state
            .ui_state
            .pointer_in_use
            .load(std::sync::atomic::Ordering::Relaxed);

        if !pointer_in_use {
            if let Some(CameraResult {
                view,
                proj,
                viewport,
                eye,
            }) = self.camera_result.clone()
            {
                if let WindowEvent::MouseInput { button, state, .. } = event {
                    if *state == ElementState::Pressed {
                        if let Some((x, y)) = global_state.ctx.mouse_position {
                            let now = std::time::Instant::now();

                            let ray =
                                rether::picking::Ray::from_view(viewport, (x, y), view, proj, eye);

                            {
                                let server = global_state.viewer.toolpath_server.read();

                                let model = server.root_hitbox().check_hit(&ray);

                                if let Some(_model) = model {
                                    let focused = server.get_focused().unwrap_or("");

                                    if let Some(toolpath) = server.get_toolpath(focused) {
                                        println!("PickingAdapter: Focused: {}", focused);

                                        toolpath.handle.clicked(ClickEvent {
                                            action: rether::picking::interact::Action::Mouse(
                                                *button,
                                            ),
                                        })
                                    }
                                }

                                let server = global_state.viewer.model_server.read();

                                let model = server.root_hitbox().check_hit(&ray);

                                if let Some(model) = model {
                                    model.clicked(ClickEvent {
                                        action: rether::picking::interact::Action::Mouse(*button),
                                    });
                                }
                            }

                            println!("PickingAdapter: Picking took {:?}", now.elapsed());
                        }
                    }

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
            }
        } else if let WindowEvent::MouseInput { button, state, .. } = event {
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
    }

    fn handle_device_event(
        &mut self,
        event: &DeviceEvent,
        _id: winit::event::DeviceId,
        _wgpu_context: &WgpuContext,
        state: GlobalState<RootEvent>,
    ) {
        let pointer_in_use = state
            .ui_state
            .pointer_in_use
            .load(std::sync::atomic::Ordering::Relaxed);

        if !pointer_in_use {
            if let DeviceEvent::MouseMotion { delta } = event {
                if self.state.is_drag_left {
                    println!("PickingAdapter: Dragging Left Click");
                }

                if self.state.is_drag_right {
                    println!("PickingAdapter: Dragging Right Click");
                }
            }
        }
    }
}

impl<'a> Adapter<'a, RootEvent, PickingState, (), &CameraResult, PickingEvent> for PickingAdapter {
    fn create(_wgpu_context: &WgpuContext) -> AdapterCreation<PickingState, PickingEvent, Self> {
        let state = PickingState {
            is_drag_left: false,
            is_drag_right: false,
        };

        let (reader, writer) = create_event_bundle::<PickingEvent>();

        (
            state.clone(),
            writer,
            PickingAdapter {
                handles: vec![],

                camera_result: None,
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
            PickingEvent::Pick => {}
        }
    }
}
