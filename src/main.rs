/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use model::gcode::GCode;
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
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

use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),

        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

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

    println!("Mesh: {:?}", cpu_mesh);

    let mut sphere = Gm::new(
        Mesh::new(&context, &cpu_mesh.0),
        PhysicalMaterial::new(&context, &CpuMaterial::default()),
    );

    if let Some(vec) = cpu_mesh.1 {
        sphere.set_transformation(Mat4::from_translation(Vector3::new(-vec.x, -vec.y, -vec.z)));
    }

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &sphere, &[&light0, &light1]);

        println!("Elapsed: {}", 1000.0 / frame_input.elapsed_time);

        FrameOutput::default()
    });
}
