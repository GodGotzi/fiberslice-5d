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

use application::Application;
use three_d::*;
use window::build_window;

#[tokio::main]
async fn main() {
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
        .near(0.001)
        .far(10000.0)
        .build()
        .expect("Failed to create camera");

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 0.00001, 1000.0);
    camera.zoom_towards(&vec3(0.0, 0.0, 0.0), -400.0, 0.00001, 1000.0);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    let mut assets = three_d_asset::io::load(&["assets/without-textures.glb"]).unwrap();

    let model: three_d_asset::Model = assets.deserialize("without-textures.glb").unwrap();
    //model.materials.iter_mut().for_each(|m| m.albedo = Srgba::BLUE);

    let mut model = Model::<PhysicalMaterial>::new(&context, &model)
        .unwrap()
        .remove(0);

    let scale = Mat4::from_scale(1.0);
    let rotation = Mat4::from_angle_y(degrees(90.0))
        .concat(&Mat4::from_angle_x(degrees(90.0)))
        .concat(&Mat4::from_angle_z(degrees(45.0)));

    let translation = Mat4::from_translation(vec3(0.0, 0.0, 0.0));
    model.set_transformation(translation * rotation * scale);

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
                control.handle_events(&mut camera, &mut frame_input.events);
            }

            println!("{}", frame_input.elapsed_time);

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
            
                camera.set_viewport(viewport);
            }

            // Then, based on whether or not we render the instanced cubes, collect the renderable
            // objects.
            let screen = frame_input.screen();
            screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
            screen.write(|| {
                model.render(&camera, &[&light0, &light1]);
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
        _ => {}
    });
}
