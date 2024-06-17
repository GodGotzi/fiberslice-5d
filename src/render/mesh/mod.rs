use crate::{geometry::BoundingBox, picking::Pickable, render::buffer::BufferLocation};

use super::{buffer::RenderBuffers, vertex::Vertex, Shared};

#[derive(Debug, Clone)]
pub enum TempMesh {
    Static {
        vertices: Vec<Vertex>,
        sub_meshes: Vec<SubTempMesh>,
    },
    Interactive {
        vertices: Vec<Vertex>,
        sub_meshes: Vec<SubTempMesh>,
        raw_box: BoundingBox,
        context: Shared<Box<dyn Pickable>>,
    },
}

#[derive(Debug, Clone)]
pub enum SubTempMesh {
    Static {
        sub_meshes: Vec<SubTempMesh>,
    },
    Interactive {
        sub_meshes: Vec<SubTempMesh>,
        raw_box: BoundingBox,
        context: Shared<Box<dyn Pickable>>,
    },
}

#[derive(Debug, Clone)]
pub enum MeshHandle {
    Static {
        location: BufferLocation,
        sub_meshes: Vec<MeshHandle>,
    },
    Interactive {
        location: BufferLocation,
        sub_meshes: Vec<MeshHandle>,
        raw_box: BoundingBox,
        context: Shared<Box<dyn Pickable>>,
    },
}

pub fn load_mesh(buffers: &mut RenderBuffers, mesh: TempMesh) {
    todo!()
}
