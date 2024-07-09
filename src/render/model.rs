use crate::{
    picking::{hitbox::Hitbox, Pickable},
    render::buffer::BufferLocation,
};

use super::{Shared, SharedMut};

#[derive(Debug, Clone)]
pub enum Model<T: bytemuck::Pod + bytemuck::Zeroable + Clone> {
    Static {
        vertices: Vec<T>,
        sub_meshes: Vec<SubModel>,
        location: BufferLocation,
    },
    Interactive {
        vertices: Vec<T>,
        sub_meshes: Vec<SubModel>,
        location: BufferLocation,
        raw_box: SharedMut<Box<dyn Hitbox>>,
        context: Shared<Box<dyn Pickable>>,
    },
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone> Model<T> {
    pub fn location(&self) -> &BufferLocation {
        match self {
            Self::Static { location, .. } => location,
            Self::Interactive { location, .. } => location,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubModel {
    Static {
        sub_meshes: Vec<SubModel>,
        location: BufferLocation,
    },
    Interactive {
        sub_meshes: Vec<SubModel>,
        location: BufferLocation,
        raw_box: SharedMut<Box<dyn Hitbox>>,
        context: Shared<Box<dyn Pickable>>,
    },
}

impl SubModel {
    pub fn location(&self) -> &BufferLocation {
        match self {
            Self::Static { location, .. } => location,
            Self::Interactive { location, .. } => location,
        }
    }

    pub fn hitbox(&self) -> Option<&SharedMut<Box<dyn Hitbox>>> {
        match self {
            Self::Static { .. } => None,
            Self::Interactive { raw_box, .. } => Some(raw_box),
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
        raw_box: SharedMut<Box<dyn Hitbox>>,
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

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone> Model<T> {
    pub fn into_handle(self, offset: usize) -> MeshHandle {
        match self {
            Self::Static {
                sub_meshes,
                location,
                ..
            } => MeshHandle::Static {
                location: BufferLocation {
                    offset: location.offset + offset,
                    size: location.size,
                },
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into_handle(offset))
                    .collect(),
            },
            Self::Interactive {
                sub_meshes,
                location,
                raw_box,
                context,
                ..
            } => MeshHandle::Interactive {
                location: BufferLocation {
                    offset: location.offset + offset,
                    size: location.size,
                },
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into_handle(offset))
                    .collect(),
                raw_box,
                context: context.clone(),
            },
        }
    }
}

impl SubModel {
    fn into_handle(self, offset: usize) -> MeshHandle {
        match self {
            Self::Static {
                sub_meshes,
                location,
            } => MeshHandle::Static {
                location: BufferLocation {
                    offset: location.offset + offset,
                    size: location.size,
                },
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into_handle(offset))
                    .collect(),
            },
            Self::Interactive {
                sub_meshes,
                location,
                raw_box,
                context,
            } => MeshHandle::Interactive {
                location: BufferLocation {
                    offset: location.offset + offset,
                    size: location.size,
                },
                sub_meshes: sub_meshes
                    .into_iter()
                    .map(|sub_mesh| sub_mesh.into_handle(offset))
                    .collect(),
                raw_box,
                context: context.clone(),
            },
        }
    }
}
