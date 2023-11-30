use model::gcode::GCode;
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
use prelude::*;
/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/
use three_d::*;

mod actions;
mod api;
mod config;
mod environment;
mod error;
mod event;
mod model;
mod prelude;
mod render;
mod settings;
mod shortcut;
mod slicer;
mod tests;
mod ui;
mod view;
mod window;

use window::build_window;
use winit::event_loop;

use crate::prelude::{FrameHandle, RenderHandle};

pub fn main() {
    let event_loop = event_loop::EventLoop::new();
    let window = build_window(&event_loop).unwrap();
    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();

    let shared_state = SharedState::default();

    let ui_adapter = ui::UiAdapter::from_context(&context);
    let environment_adapter = environment::EnvironmentAdapter::from_context(&context);
    let render_adapter = render::RenderAdapter::from_context(&context);

    let cpu_model = create_toolpath(&context);
    window.set_visible(true);

    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let frame_input = frame_input_generator.generate(&context);

            let ui_result = ui_adapter
                .handle_frame(&frame_input)
                .expect("Failed to handle frame");

            if !ui_result.pointer_use.unwrap_or(false) {
                let mut events = frame_input
                    .events
                    .clone()
                    .into_iter()
                    .filter(|event| {
                        let position = match event {
                            Event::MousePress { position, .. } => position,
                            Event::MouseRelease { position, .. } => position,
                            Event::MouseMotion { position, .. } => position,
                            Event::MouseWheel { position, .. } => position,
                            _ => return true,
                        };

                        environment.camera().viewport().contains(position)
                    })
                    .collect::<Vec<Event>>();

                environment.handle_camera_events(&mut events);
            }

            environment.frame(&frame_input, &data);

            let screen: RenderTarget<'_> = frame_input.screen();
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));

            screen.write(|| {
                cpu_model.render(environment.camera(), &environment.lights());
                ui_adapter.handle();
            });

            context.swap_buffers().unwrap();
            control_flow.set_poll();
            window.request_redraw();
        }
        winit::event::Event::WindowEvent { ref event, .. } => {
            frame_input_generator.handle_winit_window_event(event);
            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    context.resize(*physical_size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    context.resize(**new_inner_size);
                }
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => (),
            }
        }
        _ => {}
    });
}

pub fn create_toolpath(context: &Context) -> Gm<Mesh, PhysicalMaterial> {
    let nfd = Nfd::new().unwrap();
    let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

    let cpu_mesh = match result {
        DialogResult::Ok(path) => {
            let content = std::fs::read_to_string(path).unwrap();
            let gcode: GCode = content.try_into().unwrap();

            let toolpath = gcode.into_mesh();

            (toolpath.0, toolpath.1.center)
        }
        _ => (CpuMesh::cube(), None),
    };

    let mut cpu_model = Gm::new(
        Mesh::new(context, &cpu_mesh.0),
        PhysicalMaterial::new(context, &CpuMaterial::default()),
    );

    if let Some(vec) = cpu_mesh.1 {
        cpu_model.set_transformation(Mat4::from_translation(Vector3::new(-vec.x, -vec.y, -vec.z)));
    }

    cpu_model
}
