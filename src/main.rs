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
mod prelude;
mod utils;
mod view;
mod window;

use std::time::Instant;

use application::Application;
use three_d::*;
use window::build_window;

fn main() {
    let mut application = Application::new();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = build_window(&event_loop).expect("Failed to build window");
    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();

    let mut camera = crate::view::camera::CameraBuilder::new()
        .viewport(Viewport::new_at_origo(
            config::default::WINDOW_S.width as u32,
            config::default::WINDOW_S.height as u32,
        ))
        .position(vec3(60.00, 50.0, 60.0))
        .target(vec3(0.0, 0.0, 0.0))
        .up(vec3(0.0, 1.0, 0.0))
        .fov(degrees(45.0))
        .near(0.1)
        .far(10000.0)
        .build()
        .expect("Failed to create camera");

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    // Instanced mesh object, initialise with empty instances.
    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 128,
                    g: 128,
                    b: 128,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    instanced_mesh.set_animation(|time| Mat4::from_angle_x(Rad(time)));

    // Event loop
    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    let mut gui = three_d::GUI::new(&context);
    let side_count = 100;

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let now = Instant::now();
            let mut frame_input = frame_input_generator.generate(&context);

            let cloned_events = &mut frame_input.events.clone();

            let mut ui_use = None;

            gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |ctx| {
                    ui_use = Some(ctx.is_using_pointer());
                    application.ui_frame(ctx);
                },
            );

            if !ui_use.unwrap() {
                control.handle_events(&mut camera, cloned_events);
            }

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

            camera.set_viewport(viewport);

            // Camera control must be after the gui update.

            let count = side_count * side_count * side_count;
            if instanced_mesh.instance_count() != count as u32 {
                instanced_mesh.set_instances(&Instances {
                    transformations: (0..count)
                        .map(|i| {
                            let x = (i % side_count) as f32;
                            let y = ((i as f32 / side_count as f32).floor() as usize % side_count)
                                as f32;
                            let z = (i as f32 / side_count.pow(2) as f32).floor();
                            Mat4::from_translation(
                                3.0 * vec3(x, y, z)
                                    - 1.5 * (side_count as f32) * vec3(1.0, 1.0, 1.0),
                            )
                        })
                        .collect(),
                    ..Default::default()
                });
            }

            let time = (frame_input.accumulated_time * 0.001) as f32;
            instanced_mesh.animate(time);

            // Then, based on whether or not we render the instanced cubes, collect the renderable
            // objects.
            let screen = frame_input.screen();
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
            screen.render(&camera, &instanced_mesh, &[&light0, &light1]);

            screen.write(|| gui.render());

            context.swap_buffers().unwrap();
            control_flow.set_poll();

            println!("FPS: {}", 1.0 / now.elapsed().as_secs_f64());
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
