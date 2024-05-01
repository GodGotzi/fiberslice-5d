/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/
use model::gcode::{self, DisplaySettings, MeshSettings};
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};

use prelude::{Adapter, SharedState};

mod actions;
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
mod tests;
mod tools;
mod ui;
mod view;
mod window;

use winit::event_loop::EventLoop;

use crate::prelude::FrameHandle;

#[tokio::main]
async fn main() {
    let event_loop = EventLoop::new();

    let mut window_handler = window::WindowHandler::from_event_loop(&event_loop);

    //let settings = SharedMut::from_inner(settings::Settings { diameter: 0.45 });
    let mesh_settings = MeshSettings {};
    let display_settings = DisplaySettings {
        diameter: 0.45,
        horizontal: 0.425,
        vertical: 0.325,
    };

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

    ui_adapter.init(&shared_state);

    //let cpu_model = create_toolpath(&context);
    window_handler.init();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window_handler.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let frame_input = window_handler.next_frame_input();

            shared_state
                .handle_frame(&frame_input, ())
                .expect("Failed to handle frame");

            let ui_output = ui_adapter.handle_frame(&frame_input, &shared_state);

            environment_adapter
                .handle_frame(&frame_input, (ui_adapter.share_state(), &ui_output))
                .expect("Failed to handle frame");

            render_adapter
                .handle_frame(
                    &frame_input,
                    (environment_adapter.share_environment(), &ui_output),
                )
                .expect("Failed to handle frame");

            picking_adapter
                .handle_frame(&frame_input, render_adapter.share_state())
                .expect("Failed to handle frame");

            ui_adapter.handle_events();
            environment_adapter.handle_events();
            render_adapter.handle_events();
            picking_adapter.handle_events();

            window_handler.borrow_context().swap_buffers().unwrap();
            control_flow.set_poll();
            window_handler.request_redraw();
        }
        winit::event::Event::WindowEvent { ref event, .. } => {
            window_handler.handle_winit_event(event, control_flow);
        }
        _ => {}
    });
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
