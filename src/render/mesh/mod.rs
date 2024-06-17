use crate::{geometry::BoundingBox, picking::Pickable, render::buffer::BufferLocation};

use super::{buffer::BufferType, vertex::Vertex, Shared};

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

pub trait MeshKit {
    fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        location: BufferLocation,
        mesh: TempMesh,
    ) -> Option<MeshHandle>;

    fn upload_renew(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        location: BufferLocation,
        mesh: TempMesh,
    ) -> Option<MeshHandle>;

    fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, handle: &MeshHandle);
}
