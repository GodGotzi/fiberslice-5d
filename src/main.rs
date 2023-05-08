mod window_builder;
mod fiberslice;

use std::iter;
use std::time::Instant;

use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use wgpu::{InstanceDescriptor, TextureFormat};
use winit::event::Event::*;
use winit::event_loop::{EventLoop};
use winit::window::Window;

use window_builder::{create_winit_window, Event};
use fiberslice::wgpu::request_wgpu_adapter;
use fiberslice::utils::Creation;
use fiberslice::wgpu::request_wgpu_device;
use fiberslice::window::WindowEventObserver;
use fiberslice::window::FiberSliceWindow;

fn main() {
    let event_loop = EventLoop::create();
    let window: Window = create_winit_window(&event_loop);

    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(&window).unwrap() };

    let adapter = request_wgpu_adapter(&instance, &surface).unwrap();

    let (device, queue) = request_wgpu_device(&adapter).unwrap();

    let size = window.inner_size();


    let texture_format = TextureFormat::Bgra8UnormSrgb;
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: Default::default(),
        view_formats: Default::default(),
    };
    surface.configure(&device, &surface_config);

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut egui_rpass = RenderPass::new(&device, texture_format, 1);

    let mut demo_app = egui_demo_lib::DemoWindows::default();
    let start_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        platform.handle_event(&event);

        let mut fiberslice_window = FiberSliceWindow::new(&window, &device, &surface, &mut surface_config);
        let window_event_observer = WindowEventObserver::new(&window);

        match event {
            RedrawRequested(..) => {
                platform.update_time(start_time.elapsed().as_secs_f64());

                let output_frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(wgpu::SurfaceError::Outdated) => {
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

                platform.begin_frame();

                demo_app.ui(&platform.context());

                let full_output = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(full_output.shapes);

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });

                let screen_descriptor = ScreenDescriptor {
                    physical_width: fiberslice_window.get_surface_config().width,
                    physical_height: fiberslice_window.get_surface_config().height,
                    scale_factor: window.scale_factor() as f32,
                };
                let tdelta: egui::TexturesDelta = full_output.textures_delta;
                egui_rpass
                    .add_textures(&device, &queue, &tdelta)
                    .expect("add texture ok");
                egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

                egui_rpass
                    .execute(
                        &mut encoder,
                        &output_view,
                        &paint_jobs,
                        &screen_descriptor,
                        Some(wgpu::Color::BLACK),
                    )
                    .unwrap();

                queue.submit(iter::once(encoder.finish()));

                output_frame.present();

                egui_rpass
                    .remove_textures(tdelta)
                    .expect("remove texture ok");
            }
            MainEventsCleared | UserEvent(Event::RequestRedraw) => {
                window.request_redraw();
            }
            WindowEvent { event, .. } => {
                window_event_observer.call_event(event, &mut fiberslice_window, control_flow);
            },
            _ => (),
        }
    });
}