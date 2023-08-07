/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

mod application;
mod config;
mod error;
mod gui;
mod math;
mod model;
mod prelude;
mod setup;
mod slicer;
mod utils;
mod view;
mod window;

use application::Application;
use three_d::*;
use view::{buffer::ObjectBuffer, environment, visualization::Visualizer};
use window::build_window;

#[tokio::main]
async fn main() {
    let mut application = Application::new();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = build_window(&event_loop).expect("Failed to build window");

    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();
    let mut buffer = ObjectBuffer::new();

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 0.00001, 1000.0);

    let mut environment = environment::Environment::new(&context);

    {
        let model: three_d_asset::Model =
            three_d_asset::io::load_and_deserialize("assets/without-textures.glb").unwrap();

        let mut model = Model::<PhysicalMaterial>::new(&context, &model)
            .unwrap()
            .remove(0);

        let scale = Mat4::from_scale(1.0);
        let rotation = Mat4::from_angle_y(degrees(90.0))
            .concat(&Mat4::from_angle_x(degrees(90.0)))
            .concat(&Mat4::from_angle_z(degrees(45.0)));

        let translation = Mat4::from_translation(vec3(0.0, 0.0, 0.0));
        model.set_transformation(translation * rotation * scale);

        buffer.add_object("PRINT_BED", model);
    }

    // Event loop
    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    let mut gui = three_d::GUI::new(&context);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            let mut frame_input = frame_input_generator.generate(&context);

            let mut ui_use = None;

            let mut events = frame_input.events.clone();

            gui.update(
                &mut events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |ctx| {
                    ui_use = Some(ctx.is_using_pointer());
                    application.ui_frame(ctx);
                },
            );

            if !ui_use.unwrap() {
                control.handle_events(environment.camera_mut(), &mut frame_input.events);
            }

            if frame_input.viewport.height != 0 && frame_input.viewport.width != 0 {
                let viewport = Viewport {
                    x: (application.boundaries().toolbar.width() * frame_input.device_pixel_ratio)
                        as i32,
                    y: ((application.boundaries().taskbar.height()
                        + application.boundaries().modebar.height())
                        * frame_input.device_pixel_ratio) as i32,
                    width: frame_input.viewport.width
                        - ((application.boundaries().toolbar.width()
                            + application.boundaries().settingsbar.width())
                            * frame_input.device_pixel_ratio) as u32,
                    height: frame_input.viewport.height
                        - ((application.boundaries().taskbar.height()
                            + application.boundaries().modebar.height()
                            + application.boundaries().menubar.height())
                            * frame_input.device_pixel_ratio) as u32,
                };

                environment.camera_mut().set_viewport(viewport);
            }

            // Then, based on whether or not we render the instanced cubes, collect the renderable
            // objects.
            let screen: RenderTarget<'_> = frame_input.screen();
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));

            application
                .visualizer()
                .gcode()
                .render(&context, &screen, &environment)
                .unwrap();

            screen.write(|| {
                buffer.render(&environment);
                gui.render();
            });

            context.swap_buffers().unwrap();
            control_flow.set_poll();

            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
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
        winit::event::Event::LoopDestroyed => {
            application.save();
            application.kill();
        }
        _ => {}
    });
}
