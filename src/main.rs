use model::gcode::GCode;
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
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
mod error;
mod event;
mod math;
mod model;
mod prelude;
mod settings;
mod shortcut;
mod slicer;
mod tests;
mod ui;
mod view;
mod window;

use ui::screen::Screen;
use window::build_window;
use winit::event_loop;

use crate::ui::SuperComponent;

pub fn main() {
    let event_loop = event_loop::EventLoop::new();
    let window = build_window(&event_loop).unwrap();

    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();

    let mut data = ui::data::UiData::default();
    let mut screen = Screen::new();

    let mut environment = view::environment::Environment::new(&context);
    let mut gui = three_d::GUI::new(&context);

    let cpu_model = create_toolpath(&context);

    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let mut frame_input = frame_input_generator.generate(&context);

            environment.handle_camera_events(&mut frame_input.events);

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(environment.camera(), &cpu_model, &environment.lights());

            gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    screen.show(gui_context, &mut data);
                },
            );

            println!("Elapsed: {}", 1000.0 / frame_input.elapsed_time);

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

            let toolpath = gcode.into_cpu_mesh();

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
