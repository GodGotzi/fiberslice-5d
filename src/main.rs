/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use model::gcode::{self, parser, DisplaySettings, GCode, MeshSettings, PrintPart};
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
use settings::tree::QuickSettings;
use std::{sync::Arc, time::Instant};

use prelude::{Adapter, FrameHandle, GlobalContext, SharedMut, WgpuContext};

mod api;
mod config;
mod control;
mod environment;
mod error;
mod model;
mod picking;
mod prelude;
mod render;
mod settings;
mod shortcut;
mod slicer;
mod tools;
mod ui;

mod window;

use winit::{
    error::EventLoopError,
    event_loop::{EventLoopBuilder, EventLoopProxy},
};

#[derive(Debug, Clone)]
pub enum RootEvent {
    UiEvent(ui::UiEvent),
    PickingEvent(picking::PickingEvent),
    RenderEvent(render::RenderEvent),
}

#[derive(Debug, Clone)]
pub struct GlobalState<T: 'static> {
    pub proxy: EventLoopProxy<T>,
    pub ui_state: ui::UiState,

    pub fiber_settings: SharedMut<QuickSettings>,
    pub topology_settings: SharedMut<QuickSettings>,
    pub view_settings: SharedMut<QuickSettings>,

    pub ctx: GlobalContext,
}

#[tokio::main]
async fn main() -> Result<(), EventLoopError> {
    let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
    puffin::set_scopes_on(true);

    let event_loop = EventLoopBuilder::<RootEvent>::with_user_event()
        .build()
        .unwrap();
    let window = Arc::new(window::build_window(&event_loop).unwrap());

    // let settings = SharedMut::from_inner(settings::Settings { diameter: 0.45 });
    let mesh_settings = MeshSettings {};
    let display_settings = DisplaySettings {
        diameter: 0.45,
        horizontal: 0.425,
        vertical: 0.325,
    };

    let mut wgpu_context = WgpuContext::new(window.clone()).unwrap();

    let mut render_adapter = render::RenderAdapter::from_context(&wgpu_context).1;

    let (ui_state, mut ui_adapter) = ui::UiAdapter::from_context(&wgpu_context);

    // let mut picking_adapter = picking::PickingAdapter::from_context(&wgpu_context);
    // let mut environment_adapter = environment::EnvironmentAdapter::from_context(&wgpu_context);

    let proxy = event_loop.create_proxy();

    let nfd = Nfd::new().unwrap();
    let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

    match result {
        DialogResult::Ok(path) => {
            let content = std::fs::read_to_string(path).unwrap();
            let gcode: gcode::GCode = gcode::parser::parse_content(&content).unwrap();

            let workpiece = gcode::PrintPart::from_gcode(
                (content.lines(), gcode),
                &mesh_settings,
                &display_settings,
            );

            proxy
                .send_event(RootEvent::RenderEvent(
                    render::RenderEvent::UpdateVertexBuffer(workpiece.vertices()),
                ))
                .unwrap();

            /*
                        let mut cpu_model = Gm::new(
                Mesh::new(context, &cpu_mesh.0),
                PhysicalMaterial::new(context, &CpuMaterial::default()),
            );

            if let Some(vec) = cpu_mesh.1 {
                cpu_model.set_transformation(Mat4::from_translation(Vector3::new(
                    -vec.x, -vec.y, -vec.z,
                )));
            }

            cpu_model

             */
        }
        _ => {
            println!("No file selected")
        }
    }

    let mut global_state = GlobalState {
        proxy,
        ui_state,
        fiber_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
        topology_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
        view_settings: SharedMut::from_inner(QuickSettings::new("settings/main.yaml")),
        ctx: GlobalContext::default(),
    };

    let start_time = Instant::now();
    event_loop.run(move |event, loop_target| {
        if let winit::event::Event::WindowEvent {
            event: winit_event, ..
        } = event.clone()
        {
            match winit_event {
                winit::event::WindowEvent::RedrawRequested => {
                    global_state.ctx.begin_frame();
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
                _ => {
                    window.request_redraw();
                }
            }
        }

        let ui_output = ui_adapter
            .handle_frame(&event, start_time, &wgpu_context, global_state.clone())
            .unwrap();

        render_adapter
            .handle_frame(
                &event,
                start_time,
                &wgpu_context,
                (global_state.clone(), ui_output),
            )
            .unwrap();

        /*
        environment_adapter
            .handle_frame(&event, start_time, &wgpu_context, global_state.clone())
            .unwrap();

        picking_adapter
            .handle_frame(&event, start_time, &wgpu_context, global_state.clone())
            .unwrap();
        */

        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::RedrawRequested,
            ..
        } = event.clone()
        {
            window.set_visible(true);
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
