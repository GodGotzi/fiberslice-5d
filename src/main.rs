/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use camera::CameraEvent;
use log::{info, LevelFilter};
use picking::PickingEvent;
use settings::tree::QuickSettings;
use std::{sync::Arc, time::Instant};
use ui::UiEvent;

use prelude::{Adapter, EventWriter, FrameHandle, GlobalContext, SharedMut, Viewport, WgpuContext};

mod api;
mod camera;
mod config;
mod control;
mod env;
mod error;
mod geometry;
mod model;
mod picking;
mod prelude;
mod render;
mod settings;
mod shortcut;
mod slicer;
mod tools;
mod ui;
mod viewer;
mod window;

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{EventLoop, EventLoopProxy},
};

#[derive(Debug, Clone)]
pub enum RootEvent {
    Exit,
}

#[derive(Debug, Clone)]
pub struct GlobalState<T: 'static> {
    pub proxy: EventLoopProxy<T>,

    pub picking_state: picking::PickingState,
    pub picking_event_writer: EventWriter<PickingEvent>,

    pub ui_state: ui::UiState,
    pub ui_event_writer: EventWriter<UiEvent>,

    pub camera_event_writer: EventWriter<CameraEvent>,

    pub toolpath_server: SharedMut<viewer::part_server::ToolpathServer>,
    pub widget_server: SharedMut<viewer::widget_server::WidgetServer>,

    pub fiber_settings: SharedMut<QuickSettings>,
    pub topology_settings: SharedMut<QuickSettings>,
    pub view_settings: SharedMut<QuickSettings>,

    pub camera_controller: SharedMut<camera::camera_controller::CameraController>,
    pub viewport: SharedMut<Viewport>,

    pub ctx: GlobalContext,
}

#[tokio::main]
async fn main() -> Result<(), EventLoopError> {
    let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");

    #[cfg(debug_assertions)]
    puffin::set_scopes_on(true);

    #[cfg(debug_assertions)]
    simple_logging::log_to_file("app.log", LevelFilter::Info).unwrap();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("Starting up version {}", VERSION);

    let event_loop: EventLoop<RootEvent> = EventLoop::with_user_event().build().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let mut application = Application {
        proxy: event_loop.create_proxy(),
        state: None,
    };

    event_loop.run_app(&mut application)

    /*
    event_loop.run(move |event, loop_target| {
        match event.clone() {
            winit::event::Event::WindowEvent {
                event: winit_event, ..
            } => match winit_event {
                winit::event::WindowEvent::RedrawRequested => {
                    global_state.ctx.begin_frame();
                    puffin::GlobalProfiler::lock().new_frame();
                }
                winit::event::WindowEvent::Resized(size) => {
                    resize_surface(&mut wgpu_context, size);

                    wgpu_context.window.request_redraw();
                }
                winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                    let size = wgpu_context.window.inner_size();

                    resize_surface(&mut wgpu_context, size);

                    wgpu_context.window.request_redraw();
                }
                winit::event::WindowEvent::CloseRequested => {
                    loop_target.exit();
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    global_state.ctx.mouse_position = Some((position.x as f32, position.y as f32));
                    window.request_redraw();
                }
                _ => {
                    window.request_redraw();
                }
            },
            winit::event::Event::UserEvent(RootEvent::Exit) => {
                loop_target.exit();
            }
            _ => {}
        }

        let (ui_output, viewport) = ui_adapter
            .handle_frame(&event, start_time, &wgpu_context, global_state.clone())
            .unwrap();

        let camera_result = camera_adapter
            .handle_frame(
                &event,
                start_time,
                &wgpu_context,
                (global_state.clone(), viewport),
            )
            .unwrap();

        render_adapter
            .handle_frame(
                &event,
                start_time,
                &wgpu_context,
                (global_state.clone(), ui_output, &camera_result),
            )
            .unwrap();

        picking_adapter
            .handle_frame(
                &event,
                start_time,
                &wgpu_context,
                (global_state.clone(), &camera_result),
            )
            .unwrap();

        global_state
            .toolpath_server
            .write()
            .update(global_state.clone(), &wgpu_context)
            .unwrap();

        ui_adapter.handle_events(&wgpu_context, &global_state);
        camera_adapter.handle_events(&wgpu_context, &global_state);
        render_adapter.handle_events(&wgpu_context, &global_state);
        picking_adapter.handle_events(&wgpu_context, &global_state);

        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::RedrawRequested,
            ..
        } = event
        {
            global_state.ctx.end_frame();
        }
    })
    */
}

struct ApplicationState {
    window: Arc<winit::window::Window>,
    wgpu_context: WgpuContext,
    global_state: GlobalState<RootEvent>,

    ui_adapter: ui::UiAdapter,
    camera_adapter: camera::CameraAdapter,
    render_adapter: render::RenderAdapter,
    picking_adapter: picking::PickingAdapter,

    start_time: Instant,
}

impl ApplicationState {
    fn update(&mut self) {
        self.global_state
            .toolpath_server
            .write()
            .update(self.global_state.clone(), &self.wgpu_context)
            .unwrap();

        self.ui_adapter.update(self.start_time);

        self.camera_adapter.update(self.start_time);

        self.render_adapter.update(self.start_time);

        self.picking_adapter.update(self.start_time);

        self.ui_adapter
            .handle_events(&self.wgpu_context, &self.global_state);

        self.camera_adapter
            .handle_events(&self.wgpu_context, &self.global_state);

        self.render_adapter
            .handle_events(&self.wgpu_context, &self.global_state);

        self.picking_adapter
            .handle_events(&self.wgpu_context, &self.global_state);
    }

    fn handle_frame(&mut self) {
        let (ui_output, viewport) = self
            .ui_adapter
            .handle_frame(&self.wgpu_context, self.global_state.clone(), ())
            .expect("Failed to handle frame");

        let camera_result = self
            .camera_adapter
            .handle_frame(&self.wgpu_context, self.global_state.clone(), viewport)
            .expect("Failed to handle frame");

        self.render_adapter
            .handle_frame(
                &self.wgpu_context,
                self.global_state.clone(),
                (Some(ui_output), &camera_result),
            )
            .expect("Failed to handle frame");

        self.picking_adapter
            .handle_frame(
                &self.wgpu_context,
                self.global_state.clone(),
                &camera_result,
            )
            .expect("Failed to handle frame");
    }

    fn handle_window_event(
        &mut self,
        event: winit::event::WindowEvent,
        window_id: winit::window::WindowId,
    ) {
        self.ui_adapter.handle_window_event(
            &event,
            window_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );

        self.camera_adapter.handle_window_event(
            &event,
            window_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );

        self.render_adapter.handle_window_event(
            &event,
            window_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );

        self.picking_adapter.handle_window_event(
            &event,
            window_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );
    }

    fn handle_device_event(
        &mut self,
        event: winit::event::DeviceEvent,
        device_id: winit::event::DeviceId,
    ) {
        self.ui_adapter.handle_device_event(
            &event,
            device_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );

        self.camera_adapter.handle_device_event(
            &event,
            device_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );

        self.render_adapter.handle_device_event(
            &event,
            device_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );

        self.picking_adapter.handle_device_event(
            &event,
            device_id,
            &self.wgpu_context,
            self.global_state.clone(),
        );
    }
}

struct Application {
    proxy: EventLoopProxy<RootEvent>,
    state: Option<ApplicationState>,
}

impl ApplicationHandler<RootEvent> for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(window::create_window(event_loop).expect("Failed to create window"));

        let wgpu_context = WgpuContext::new(window.clone()).unwrap();

        let (_, _, render_adapter) = render::RenderAdapter::create(&wgpu_context);

        let (_, camera_event_writer, camera_adapter) = camera::CameraAdapter::create(&wgpu_context);

        let (picking_state, picking_event_writer, picking_adapter) =
            picking::PickingAdapter::create(&wgpu_context);

        let (ui_state, ui_event_writer, ui_adapter) = ui::UiAdapter::create(&wgpu_context);

        let global_state = GlobalState {
            proxy: self.proxy.clone(),

            picking_state,
            picking_event_writer,

            ui_state,
            ui_event_writer,

            camera_event_writer,

            toolpath_server: SharedMut::from_inner(viewer::part_server::ToolpathServer::new(
                &wgpu_context.device,
            )),
            widget_server: SharedMut::from_inner(viewer::widget_server::WidgetServer::new(
                &wgpu_context.device,
            )),

            fiber_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
            topology_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
            view_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),

            camera_controller: SharedMut::from_inner(camera::CameraController::new(
                0.01, -2.0, 0.1,
            )),
            viewport: SharedMut::from_inner(Viewport::default()),
            ctx: GlobalContext::default(),
        };

        self.state = Some(ApplicationState {
            window,
            wgpu_context,
            global_state,

            ui_adapter,
            camera_adapter,
            render_adapter,
            picking_adapter,

            start_time: Instant::now(),
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(state) = self.state.as_mut() {
            state.handle_window_event(event.clone(), window_id);

            match event {
                winit::event::WindowEvent::RedrawRequested => {
                    state.global_state.ctx.begin_frame();
                    puffin::GlobalProfiler::lock().new_frame();

                    state.handle_frame();

                    state.global_state.ctx.end_frame();
                }
                winit::event::WindowEvent::Resized(size) => {
                    resize_surface(&mut state.wgpu_context, size);

                    state.window.request_redraw();
                }
                winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                    let size = state.wgpu_context.window.inner_size();

                    resize_surface(&mut state.wgpu_context, size);

                    state.window.request_redraw();
                }
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    state.global_state.ctx.mouse_position =
                        Some((position.x as f32, position.y as f32));
                    state.window.request_redraw();
                }
                _ => {
                    state.window.request_redraw();
                }
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(state) = self.state.as_mut() {
            state.handle_device_event(event, device_id);
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: RootEvent) {
        match event {
            RootEvent::Exit => {
                event_loop.exit();
            }
        }
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        if let Some(state) = self.state.as_mut() {
            state.update();
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("Exiting");
    }
}

fn resize_surface(wgpu_context: &mut WgpuContext, size: winit::dpi::PhysicalSize<u32>) {
    if size.width > 0 && size.height > 0 {
        wgpu_context.surface_config.width = size.width;
        wgpu_context.surface_config.height = size.height;
        wgpu_context
            .surface
            .configure(&wgpu_context.device, &wgpu_context.surface_config);
    }
}
