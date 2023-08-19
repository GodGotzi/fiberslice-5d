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
mod tests;
mod utils;
mod view;
mod window;

use application::Application;
use three_d::*;
use view::{buffer::ObjectBuffer, environment};
use window::build_window;

use utils::frame::FrameHandle;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = build_window(&event_loop).expect("Failed to build window");

    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();
    let mut application = Application::new(&window);

    let mut buffer: ObjectBuffer<dyn Object> = ObjectBuffer::new();

    let mut environment = environment::Environment::new(&context);

    test_buffer(&context, &mut application, &mut buffer);

    let mut gui = three_d::GUI::new(&context);
    window.set_visible(true);

    // Event loop
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            let mut frame_input = application.next_frame_input(&context);

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
                environment.handle_camera_events(&mut frame_input.events);
            }

            environment.frame(&frame_input, &application);

            //Render
            {
                let screen: RenderTarget<'_> = frame_input.screen();
                screen.clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0));

                screen.write(|| {
                    buffer.render(&environment);
                    gui.render();
                });
            }

            context.swap_buffers().unwrap();
            control_flow.set_poll();

            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            window.request_redraw();
        }
        winit::event::Event::WindowEvent { ref event, .. } => {
            application.handle_window_event(event, &context, control_flow);
        }
        winit::event::Event::LoopDestroyed => {
            application.save();
            application.kill();
        }
        _ => {}
    });
}

pub fn test_buffer(
    context: &WindowedContext,
    application: &mut Application,
    buffer: &mut ObjectBuffer<dyn Object>,
) {
    let environment_map =
        three_d_asset::io::load_and_deserialize("wallpapers/nebel2_wallpaper.hdr").unwrap();

    let skybox = Skybox::new_from_equirectangular(context, &environment_map);
    buffer.set_skybox(skybox);

    let model: three_d_asset::Model =
        three_d_asset::io::load_and_deserialize("assets/without-textures.glb").unwrap();

    let mut model = Model::<PhysicalMaterial>::new(context, &model)
        .unwrap()
        .remove(0);

    let scale = Mat4::from_scale(1.0);
    let rotation = Mat4::from_angle_y(degrees(90.0))
        .concat(&Mat4::from_angle_x(degrees(90.0)))
        .concat(&Mat4::from_angle_z(degrees(45.0)));

    let translation = Mat4::from_translation(vec3(0.0, 0.0, 0.0));
    model.set_transformation(translation * rotation * scale);

    buffer.add_object("PRINT_BED", Box::new(model));
}
