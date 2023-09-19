use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
use three_d::*;
use three_d_asset::TriMesh;

use crate::{
    application::AsyncAction,
    model::gcode::GCode,
    view::buffer::{HideableObject, ModelMap},
};

use std::{
    cell::Cell,
    fs,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use stl_io::Vertex;
use three_d_asset::Positions;

pub struct ModelFile(pub PathBuf);
pub struct STLFile(PathBuf);

impl From<ModelFile> for TriMesh {
    fn from(value: ModelFile) -> Self {
        match value.0.as_os_str().to_str().unwrap().split('.').last() {
            Some("stl") => {
                let stl_file = STLFile(value.0);
                stl_file.into()
            }
            _ => panic!("Unsupported file format"),
        }
    }
}

struct W(Vertex);

impl From<W> for three_d::Vector3<f32> {
    fn from(value: W) -> Self {
        Self::new(value.0[0], value.0[1], value.0[2])
    }
}

impl From<STLFile> for TriMesh {
    fn from(value: STLFile) -> Self {
        let mut stl_file = std::fs::File::open(value.0).unwrap();
        let mut reader = BufReader::new(&mut stl_file);
        let mesh = stl_io::read_stl(&mut reader).unwrap();

        let mut positions = Vec::new();

        for face in mesh.faces {
            positions.push(W(mesh.vertices[face.vertices[0]]).into());
            positions.push(W(mesh.vertices[face.vertices[1]]).into());
            positions.push(W(mesh.vertices[face.vertices[2]]).into());
        }

        let mut mesh = TriMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        };

        mesh.compute_normals();

        mesh
    }
}

pub fn import_model(
    context: three_d::Context,
    manipulator: std::sync::Arc<std::sync::Mutex<crate::view::buffer::ManipulatorHolder>>,
) {
    let _handle = tokio::spawn(async move {
        let nfd = Nfd::new().unwrap();

        // Show the dialog...
        // Note: .show() will block until the dialog is closed
        // You can also set a default path using .default_path(Path)
        let result = nfd.open_file().add_filter("STL", "stl").unwrap().show();

        let action = AsyncAction::new(Box::new(move |hashmap: ModelMap| match result {
            DialogResult::Ok(path) => {
                let file = ModelFile(path.clone());
                let mesh: TriMesh = file.into();

                hashmap.lock().unwrap().insert(
                    path.as_os_str().to_str().unwrap().to_string(),
                    HideableObject::new(Box::new(Mesh::new(&context, &mesh))),
                );
            }
            DialogResult::Err(_error_str) => {}
            DialogResult::Cancel => {}
        }));

        manipulator
            .lock()
            .unwrap()
            .model_manipulator
            .add_action(action);
    });
}

pub fn import_gcode(
    _context: three_d::Context,
    manipulator: std::sync::Arc<std::sync::Mutex<crate::view::buffer::ManipulatorHolder>>,
) {
    let _handle = tokio::spawn(async move {
        let nfd = Nfd::new().unwrap();
        let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

        let action = AsyncAction::new(Box::new(
            move |gcode_cell: Arc<Mutex<Cell<Option<GCode>>>>| match result {
                DialogResult::Ok(path) => {
                    let content = fs::read_to_string(path).unwrap();
                    let gcode: GCode = content.try_into().unwrap();
                    gcode_cell.lock().unwrap().replace(Some(gcode));
                }
                DialogResult::Err(_error_str) => {}
                DialogResult::Cancel => {}
            },
        ));

        manipulator
            .lock()
            .unwrap()
            .gcode_manipulator
            .add_action(action);
    });
}
