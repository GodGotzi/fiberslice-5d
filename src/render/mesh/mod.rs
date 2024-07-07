use crate::{geometry::BoundingBox, picking::Pickable, render::buffer::BufferLocation};

use super::{vertex::Vertex, Shared};

#[derive(Debug, Clone)]
pub enum CpuMesh<T: bytemuck::Pod + bytemuck::Zeroable + Clone> {
    Static {
        vertices: Vec<T>,
        sub_meshes: Vec<CpuSubMesh>,
        location: BufferLocation,
    },
    Interactive {
        vertices: Vec<T>,
        sub_meshes: Vec<CpuSubMesh>,
        location: BufferLocation,
        raw_box: BoundingBox,
        context: Shared<Box<dyn Pickable>>,
    },
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone> CpuMesh<T> {
    pub fn location(&self) -> &BufferLocation {
        match self {
            Self::Static { location, .. } => location,
            Self::Interactive { location, .. } => location,
        }
    }
}

pub trait CpuMeshTrait {
    fn vertices(&self) -> &Vec<Vertex>;
}

#[derive(Debug, Clone)]
pub enum CpuSubMesh {
    Static {
        sub_meshes: Vec<CpuSubMesh>,
        location: BufferLocation,
    },
    Interactive {
        sub_meshes: Vec<CpuSubMesh>,
        location: BufferLocation,
        raw_box: BoundingBox,
        context: Shared<Box<dyn Pickable>>,
    },
}

impl CpuSubMesh {
    pub fn location(&self) -> &BufferLocation {
        match self {
            Self::Static { location, .. } => location,
            Self::Interactive { location, .. } => location,
        }
    }
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

impl MeshHandle {
    pub fn location(&self) -> &BufferLocation {
        match self {
            Self::Static { location, .. } => location,
            Self::Interactive { location, .. } => location,
        }
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone> From<CpuMesh<T>> for MeshHandle {
    fn from(mesh: CpuMesh<T>) -> Self {
        match mesh {
            CpuMesh::Static {
                sub_meshes,
                location,
                ..
            } => Self::Static {
                location,
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into())
                    .collect(),
            },
            CpuMesh::Interactive {
                sub_meshes,
                location,
                raw_box,
                context,
                ..
            } => Self::Interactive {
                location,
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into())
                    .collect(),
                raw_box,
                context,
            },
        }
    }
}

impl From<CpuSubMesh> for MeshHandle {
    fn from(mesh: CpuSubMesh) -> Self {
        match mesh {
            CpuSubMesh::Static {
                sub_meshes,
                location,
            } => Self::Static {
                location,
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into())
                    .collect(),
            },
            CpuSubMesh::Interactive {
                sub_meshes,
                location,
                raw_box,
                context,
            } => Self::Interactive {
                location,
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into())
                    .collect(),
                raw_box,
                context,
            },
        }
    }
}

pub trait MeshKit<T: bytemuck::Pod + bytemuck::Zeroable + Clone> {
    fn write_mesh(&mut self, queue: &wgpu::Queue, mesh: CpuMesh<T>) -> Option<MeshHandle>;

    fn init_mesh(&mut self, device: &wgpu::Device, mesh: CpuMesh<T>) -> Option<MeshHandle>;

    fn update_mesh(&mut self, queue: &wgpu::Queue, handle: &MeshHandle, vertices: &[Vertex]);

    fn change_mesh(
        &mut self,
        queue: &wgpu::Queue,
        handle: &MeshHandle,
        change: impl Fn(&mut Vertex),
    );
}
