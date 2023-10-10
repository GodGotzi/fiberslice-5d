use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};
use three_d::*;
use three_d_asset::TriMesh;

use crate::{application::AsyncAction, model::gcode::GCode};

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
