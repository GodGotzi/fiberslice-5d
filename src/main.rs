/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use camera::CameraEvent;
use egui::ahash::HashMap;
use log::{info, LevelFilter};
use picking::PickingEvent;
use render::buffer::{
    layout::{wire::WireAllocator, WidgetAllocator},
    DynamicBuffer,
};
use settings::tree::QuickSettings;
use std::{sync::Arc, time::Instant};
use ui::UiEvent;

use prelude::{Adapter, EventWriter, FrameHandle, GlobalContext, SharedMut, WgpuContext};

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
    error::EventLoopError,
    event_loop::{EventLoopBuilder, EventLoopProxy},
};

lazy_static::lazy_static! {
    pub static ref CONFIG: HashMap<String, toml::Value> = {
        let content = include_str!("../config.toml");
        toml::from_str(content).unwrap()
    };
}

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

    pub widget_test_buffer: SharedMut<DynamicBuffer<render::vertex::Vertex, WidgetAllocator>>,
    pub widget_wire_test_buffer: SharedMut<DynamicBuffer<render::vertex::Vertex, WireAllocator>>,

    pub fiber_settings: SharedMut<QuickSettings>,
    pub topology_settings: SharedMut<QuickSettings>,
    pub view_settings: SharedMut<QuickSettings>,

    pub camera_controller: SharedMut<camera::camera_controller::CameraController>,

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

    let event_loop = EventLoopBuilder::<RootEvent>::with_user_event()
        .build()
        .unwrap();
    let window = Arc::new(window::build_window(&event_loop).unwrap());

    // let settings = SharedMut::from_inner(settings::Settings { diameter: 0.45 });

    let mut wgpu_context = WgpuContext::new(window.clone()).unwrap();

    let (_, _, mut render_adapter) = render::RenderAdapter::create(&wgpu_context);

    let (_, camera_event_writer, mut camera_adapter) = camera::CameraAdapter::create(&wgpu_context);

    let (picking_state, picking_event_writer, mut picking_adapter) =
        picking::PickingAdapter::create(&wgpu_context);

    let (ui_state, ui_event_writer, mut ui_adapter) = ui::UiAdapter::create(&wgpu_context);

    // let mut picking_adapter = picking::PickingAdapter::from_context(&wgpu_context);
    // let mut environment_adapter = environment::EnvironmentAdapter::from_context(&wgpu_context);
    let proxy = event_loop.create_proxy();

    let mut global_state = GlobalState {
        proxy,

        picking_state,
        picking_event_writer,

        ui_state,
        ui_event_writer,

        camera_event_writer,

        toolpath_server: SharedMut::from_inner(viewer::part_server::ToolpathServer::new(
            &wgpu_context.device,
        )),

        widget_test_buffer: SharedMut::from_inner(DynamicBuffer::new(
            WidgetAllocator,
            "Test Widget Buffer",
            &wgpu_context.device,
        )),
        widget_wire_test_buffer: SharedMut::from_inner(DynamicBuffer::new(
            WireAllocator,
            "Test Wire Widget Buffer",
            &wgpu_context.device,
        )),

        fiber_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
        topology_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
        view_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),

        camera_controller: SharedMut::from_inner(camera::CameraController::new(0.01, -2.0, 0.1)),
        ctx: GlobalContext::default(),
    };

    window.set_visible(true);

    let start_time = Instant::now();
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
