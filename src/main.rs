/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use model::gcode::{self, DisplaySettings, MeshSettings};
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
use std::{sync::Arc, time::Instant};

use prelude::{Adapter, FrameHandle, WgpuContext};

mod api;
mod config;
mod control;
mod environment;
mod error;
mod event;
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

use winit::{error::EventLoopError, event_loop::EventLoop};

#[tokio::main]
async fn main() -> Result<(), EventLoopError> {
    let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
    puffin::set_scopes_on(true);

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(window::build_window(&event_loop).unwrap());

    // let settings = SharedMut::from_inner(settings::Settings { diameter: 0.45 });
    let mesh_settings = MeshSettings {};
    let display_settings = DisplaySettings {
        diameter: 0.45,
        horizontal: 0.425,
        vertical: 0.325,
    };

    let mut wgpu_context = WgpuContext::new(window.clone()).unwrap();

    let (render_event_writer, mut render_adapter) =
        render::RenderAdapter::from_context(&wgpu_context);

    let (ui_event_writer, mut ui_adapter) = ui::UiAdapter::from_context(&wgpu_context);

    let (picking_event_writer, mut picking_adapter) =
        picking::PickingAdapter::from_context(&wgpu_context);

    let (environment_event_writer, mut environment_adapter) =
        environment::EnvironmentAdapter::from_context(&wgpu_context);

    let start_time = Instant::now();
    event_loop.run(move |event, loop_target| {
        // Pass the winit events to the platform integration.
        ui_adapter
            .handle_frame(&event, start_time, &wgpu_context, ())
            .unwrap();

        render_adapter.handle_events();
        ui_adapter.handle_events();

        render_adapter
            .handle_frame(&event, start_time, &wgpu_context, ())
            .unwrap();

        if let winit::event::Event::WindowEvent { event, .. } = event {
            match event {
                winit::event::WindowEvent::Resized(size) => {
                    // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                    // See: https://github.com/rust-windowing/winit/issues/208
                    // This solves an issue where the app would panic when minimizing on Windows.
                    if size.width > 0 && size.height > 0 {
                        wgpu_context.surface_config.width = size.width;
                        wgpu_context.surface_config.height = size.height;
                        wgpu_context
                            .surface
                            .configure(&wgpu_context.device, &wgpu_context.surface_config);
                    }
                }
                winit::event::WindowEvent::CloseRequested => {
                    loop_target.exit();
                }
                _ => {
                    window.request_redraw();
                }
            }
        }
    })
}

pub fn create_toolpath(
    mesh_settings: &MeshSettings,
    display_settings: &DisplaySettings,
) -> Option<gcode::PrintPart> {
    let nfd = Nfd::new().unwrap();
    let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

    match result {
        DialogResult::Ok(path) => {
            let content = std::fs::read_to_string(path).unwrap();
            let gcode: gcode::GCode = gcode::parser::parse_content(&content).unwrap();

            let workpiece = gcode::PrintPart::from_gcode(
                (content.lines(), gcode),
                mesh_settings,
                display_settings,
            );

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

            Some(workpiece)
        }
        _ => None,
    }
}
