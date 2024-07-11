use transform::{Rotate, Scale, Transform, Translate};

use crate::{
    picking::{hitbox::Hitbox, Pickable},
    render::buffer::{alloc::BufferAllocationID, BufferLocation},
};

use crate::prelude::{Shared, SharedMut};

pub mod transform;

pub type Mesh<T> = Vec<T>;

impl<T: Translate> Translate for Mesh<T> {
    fn translate(&mut self, translation: glam::Vec3) {
        for item in self.iter_mut() {
            item.translate(translation);
        }
    }
}

impl<T: Rotate> Rotate for Mesh<T> {
    fn rotate(&mut self, rotation: glam::Quat) {
        for item in self.iter_mut() {
            item.rotate(rotation);
        }
    }
}

impl<T: Scale> Scale for Mesh<T> {
    fn scale(&mut self, scale: glam::Vec3) {
        for item in self.iter_mut() {
            item.scale(scale);
        }
    }
}

#[derive(Debug)]
pub struct Geometry<T> {
    pub vertices: Mesh<T>,
}

impl<T: Translate> Translate for Geometry<T> {
    fn translate(&mut self, translation: glam::Vec3) {
        self.vertices.translate(translation);
    }
}

impl<T: Rotate> Rotate for Geometry<T> {
    fn rotate(&mut self, rotation: glam::Quat) {
        self.vertices.rotate(rotation);
    }
}

impl<T: Scale> Scale for Geometry<T> {
    fn scale(&mut self, scale: glam::Vec3) {
        self.vertices.scale(scale);
    }
}

pub trait IntoHandle<T> {
    fn req_handle(self, allocation_id: BufferAllocationID) -> T;
}

#[derive(Debug, Clone)]
pub enum ModelContext {
    Static,
    Interactive {
        box_: SharedMut<Box<dyn Hitbox>>,
        context: Shared<Box<dyn Pickable>>,
    },
}

impl ModelContext {
    pub fn expand(&mut self, model: ModelContext) {
        if let ModelContext::Interactive { box_, .. } = self {
            if let ModelContext::Interactive {
                box_: other_box, ..
            } = model
            {
                box_.write().expand(&other_box);
            }
        }
    }
}

impl Translate for ModelContext {
    fn translate(&mut self, translation: glam::Vec3) {
        if let Self::Interactive { box_, .. } = self {
            box_.write().translate(translation);
        }
    }
}

impl Rotate for ModelContext {
    fn rotate(&mut self, rotation: glam::Quat) {
        if let Self::Interactive { box_, .. } = self {
            box_.write().rotate(rotation);
        }
    }
}

impl Scale for ModelContext {
    fn scale(&mut self, scale: glam::Vec3) {
        if let Self::Interactive { box_, .. } = self {
            box_.write().scale(scale);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Model<T> {
    Root {
        geometry: Mesh<T>,
        sub_models: Vec<Model<T>>,
        ctx: ModelContext,
    },
    Node {
        location: BufferLocation,
        sub_models: Vec<Model<T>>,
        ctx: ModelContext,
    },
}

impl<T> Model<T> {
    pub fn expand(&mut self, model: Model<T>) {
        match self {
            Self::Root { sub_models, .. } => {
                sub_models.push(model);
            }
            Self::Node { sub_models, .. } => {
                sub_models.push(model);
            }
        }
    }

    pub fn push_data(&mut self, data: T) {
        match self {
            Self::Root { geometry, .. } => {
                geometry.push(data);
            }
            Self::Node { .. } => {}
        }
    }

    pub fn extend_data(&mut self, data: Vec<T>) {
        match self {
            Self::Root { geometry, .. } => geometry.extend(data),
            Self::Node { .. } => {}
        }
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone> IntoHandle<ModelHandle> for Model<T> {
    fn req_handle(self, allocation_id: BufferAllocationID) -> ModelHandle {
        match self {
            Self::Root {
                sub_models, ctx, ..
            } => ModelHandle::Root {
                id: allocation_id.clone(),
                transform: Transform::default(),
                sub_handles: sub_models
                    .into_iter()
                    .map(|model| model.req_handle(allocation_id.clone()))
                    .collect(),
                ctx,
            },
            Self::Node {
                location,
                sub_models,
                ctx,
            } => ModelHandle::Node {
                location: location.clone(),
                sub_handles: sub_models
                    .into_iter()
                    .map(|model| model.req_handle(allocation_id.clone()))
                    .collect(),
                ctx,
            },
        }
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Translate> Translate for Model<T> {
    fn translate(&mut self, translation: glam::Vec3) {
        match self {
            Self::Root {
                geometry,
                sub_models,
                ..
            } => {
                geometry.translate(translation);
                for model in sub_models.iter_mut() {
                    model.translate(translation);
                }
            }
            Self::Node { ctx, .. } => {
                ctx.translate(translation);
            }
        }
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Rotate> Rotate for Model<T> {
    fn rotate(&mut self, rotation: glam::Quat) {
        match self {
            Self::Root {
                geometry,
                sub_models,
                ..
            } => {
                geometry.rotate(rotation);
                for model in sub_models.iter_mut() {
                    model.rotate(rotation);
                }
            }
            Self::Node { ctx, .. } => {
                ctx.rotate(rotation);
            }
        }
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Clone + Scale> Scale for Model<T> {
    fn scale(&mut self, scale: glam::Vec3) {
        match self {
            Self::Root {
                geometry,
                sub_models,
                ..
            } => {
                geometry.scale(scale);
                for model in sub_models.iter_mut() {
                    model.scale(scale);
                }
            }
            Self::Node { ctx, .. } => {
                ctx.scale(scale);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModelHandle {
    Root {
        id: BufferAllocationID,
        transform: transform::Transform,
        sub_handles: Vec<ModelHandle>,
        ctx: ModelContext,
    },
    Node {
        location: BufferLocation,
        sub_handles: Vec<ModelHandle>,
        ctx: ModelContext,
    },
}

impl Translate for ModelHandle {
    fn translate(&mut self, translation: glam::Vec3) {
        match self {
            Self::Root {
                transform,
                sub_handles,
                ..
            } => {
                transform.translate(translation);
                for handle in sub_handles.iter_mut() {
                    handle.translate(translation);
                }
            }
            Self::Node { ctx, .. } => {
                ctx.translate(translation);
            }
        }
    }
}

impl Rotate for ModelHandle {
    fn rotate(&mut self, rotation: glam::Quat) {
        match self {
            Self::Root {
                transform,
                sub_handles,
                ..
            } => {
                transform.rotate(rotation);
                for handle in sub_handles.iter_mut() {
                    handle.rotate(rotation);
                }
            }
            Self::Node { ctx, .. } => {
                ctx.rotate(rotation);
            }
        }
    }
}

impl Scale for ModelHandle {
    fn scale(&mut self, scale: glam::Vec3) {
        match self {
            Self::Root {
                transform,
                sub_handles,
                ..
            } => {
                transform.scale(scale);
                for handle in sub_handles.iter_mut() {
                    handle.scale(scale);
                }
            }
            Self::Node { ctx, .. } => {
                ctx.scale(scale);
            }
        }
    }
}
