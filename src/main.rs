use std::{iter, sync::Arc, time::Instant};

use egui::FontDefinitions;
/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/
use model::gcode::{self, DisplaySettings, MeshSettings};
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};

use prelude::{Adapter, SharedState};

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use wgpu::InstanceDescriptor;

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

    // let mut window_handler = window::WindowHandler::from_event_loop(&event_loop);

    // let settings = SharedMut::from_inner(settings::Settings { diameter: 0.45 });
    let mesh_settings = MeshSettings {};
    let display_settings = DisplaySettings {
        diameter: 0.45,
        horizontal: 0.425,
        vertical: 0.325,
    };

    /*
    let workpiece = create_toolpath(&mesh_settings, &display_settings);

    let (writer_render_event, mut render_adapter) =
        render::RenderAdapter::from_context(window_handler.borrow_context());

    render_adapter.set_workpiece(workpiece.unwrap());
    render_adapter.update_from_state();

    //render_adapter.set_toolpath(toolpath);

    let (writer_environment_event, mut environment_adapter) =
        environment::EnvironmentAdapter::from_context(window_handler.borrow_context());
    let (writer_ui_event, mut ui_adapter) =
        ui::UiAdapter::from_context(window_handler.borrow_context());

    let (writer_picking_event, mut picking_adapter) =
        picking::PickingAdapter::from_context(window_handler.borrow_context());

    let mut shared_state = SharedState::new(
        writer_render_event,
        writer_environment_event,
        writer_ui_event,
        writer_picking_event,
    );
    */

    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let surface = instance.create_surface(window.clone()).unwrap();

    // WGPU 0.11+ support force fallback (if HW implementation not supported), set it to true or false (optional).
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))
    .unwrap();

    let size = window.inner_size();
    let surface_format = surface.get_capabilities(&adapter).formats[0];
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoNoVsync,
        desired_maximum_frame_latency: 1,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![surface_format],
    };
    surface.configure(&device, &surface_config);

    // We use the egui_winit_platform crate as the platform.
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width,
        physical_height: size.height,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    // We use the egui_wgpu_backend crate as the render backend.
    let mut egui_rpass = RenderPass::new(&device, surface_format, 1);

    let start_time = Instant::now();
    event_loop.run(move |event, loop_target| {
        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        println!("{:?}", loop_target.control_flow());

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::RedrawRequested => {
                    // sleep_controller.last_time = Some(Instant::now());
                    let now = Instant::now();
                    platform.update_time(start_time.elapsed().as_secs_f64());

                    let output_frame = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(wgpu::SurfaceError::Outdated) => {
                            // This error occurs when the app is minimized on Windows.
                            // Silently return here to prevent spamming the console with:
                            // "The underlying surface has changed, and therefore the swap chain must be updated"
                            return;
                        }
                        Err(e) => {
                            eprintln!("Dropped frame with error: {}", e);
                            return;
                        }
                    };
                    let output_view = output_frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Begin to draw the UI frame.
                    platform.begin_frame();

                    // Draw the demo application.
                    demo_app.ui(&platform.context());

                    // End the UI frame. We could now handle the output and draw the UI with the backend.
                    let full_output = platform.end_frame(Some(&window));
                    let paint_jobs = platform
                        .context()
                        .tessellate(full_output.shapes, full_output.pixels_per_point);

                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });

                    // Upload all resources for the GPU.
                    let screen_descriptor = ScreenDescriptor {
                        physical_width: surface_config.width,
                        physical_height: surface_config.height,
                        scale_factor: window.scale_factor() as f32,
                    };

                    let tdelta: egui::TexturesDelta = full_output.textures_delta;
                    egui_rpass
                        .add_textures(&device, &queue, &tdelta)
                        .expect("add texture ok");
                    egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

                    // Record all render passes.
                    egui_rpass
                        .execute(
                            &mut encoder,
                            &output_view,
                            &paint_jobs,
                            &screen_descriptor,
                            Some(wgpu::Color::BLACK),
                        )
                        .unwrap();
                    // Submit the commands.
                    queue.submit(iter::once(encoder.finish()));
                    // Redraw egui
                    output_frame.present();

                    egui_rpass
                        .remove_textures(tdelta)
                        .expect("remove texture ok");

                    // Support reactive on windows only, but not on linux.
                    // if _output.needs_repaint {
                    //     *control_flow = ControlFlow::Poll;
                    // } else {
                    //     *control_flow = ControlFlow::Wait;
                    // }
                    println!("Fps: {:?}", 1.0 / now.elapsed().as_secs_f64());

                    window.request_redraw();
                }
                winit::event::WindowEvent::Resized(size) => {
                    // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                    // See: https://github.com/rust-windowing/winit/issues/208
                    // This solves an issue where the app would panic when minimizing on Windows.
                    if size.width > 0 && size.height > 0 {
                        surface_config.width = size.width;
                        surface_config.height = size.height;
                        surface.configure(&device, &surface_config);
                    }
                }
                winit::event::WindowEvent::CloseRequested => {
                    loop_target.exit();
                }
                _ => {}
            },
            _ => (),
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
