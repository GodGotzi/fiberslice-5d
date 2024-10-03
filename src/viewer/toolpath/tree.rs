use std::{ops::Deref, sync::Arc};

use rether::{
    picking::{interact::InteractiveModel, Hitbox, HitboxNode},
    Rotate, Scale, Translate,
};
use wgpu::{BufferAddress, Queue};

use crate::{geometry::BoundingBox, model::Model};

use super::{mesh::PathHitbox, vertex::ToolpathVertex};

#[derive(Debug)]
pub enum ToolpathTree {
    Root {
        model: Model<ToolpathVertex>,
        bounding_box: BoundingBox,
        children: Vec<Self>,
        size: BufferAddress,
    },
    Node {
        offset: BufferAddress,
        size: BufferAddress,
        bounding_box: BoundingBox,
        children: Vec<Self>,
    },
    Path {
        offset: BufferAddress,
        size: BufferAddress,
        path_box: PathHitbox,
    },
}

impl Deref for ToolpathTree {
    type Target = Model<ToolpathVertex>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Root { model, .. } => model,
            Self::Node { .. } => panic!("Cannot deref node"),
            Self::Path { .. } => panic!("Cannot deref path"),
        }
    }
}

impl ToolpathTree {
    pub fn create_root() -> Self {
        Self::Root {
            model: Model::create(),
            children: Vec::new(),
            bounding_box: BoundingBox::default(),
            size: 0,
        }
    }

    pub fn create_root_with_children<T>(
        device: &wgpu::Device,
        queue: Arc<Queue>,
        bounding_box: BoundingBox,
        children: Vec<Self>,
    ) -> Self {
        Self::Root {
            model: Model::create(),
            children,
            bounding_box: BoundingBox::default(),
            size: 0,
        }
    }

    pub fn create_node() -> Self {
        Self::Node {
            offset: 0,
            size: 0,
            children: Vec::new(),
            bounding_box: BoundingBox::default(),
        }
    }

    pub fn create_node_with_children(children: Vec<Self>) -> Self {
        Self::Node {
            offset: 0,
            size: 0,
            children,
            bounding_box: BoundingBox::default(),
        }
    }

    pub fn create_path(path_box: PathHitbox) -> Self {
        Self::Path {
            offset: 0,
            size: 0,
            path_box,
        }
    }

    pub fn extend_root(&mut self, child: Self) {
        match self {
            Self::Root {
                children,
                bounding_box,
                size,
                ..
            } => {
                *size += child.size();
                bounding_box.expand_point(child.get_min());
                bounding_box.expand_point(child.get_max());
                children.push(child);
            }
            Self::Node { .. } => panic!("Cannot extend node"),
            Self::Path { .. } => panic!("Cannot extend path"),
        }
    }

    pub fn extend_node(&mut self, mut child: Self) {
        match self {
            Self::Root { .. } => panic!("Cannot extend root"),
            Self::Node {
                children,
                bounding_box,
                size,
                ..
            } => {
                child.set_offset(*size);
                bounding_box.expand_point(child.get_min());
                bounding_box.expand_point(child.get_max());
                *size += child.size();
                children.push(child);
            }
            Self::Path { .. } => panic!("Cannot extend path"),
        }
    }

    fn set_offset(&mut self, offset: BufferAddress) {
        match self {
            Self::Root { .. } => panic!("Cannot set offset for root"),
            Self::Node { offset: o, .. } => *o = offset,
            Self::Path { offset: o, .. } => *o = offset,
        }
    }

    fn set_size(&mut self, size: BufferAddress) {
        match self {
            Self::Root { .. } => panic!("Cannot set size for root"),
            Self::Node { size: s, .. } => *s = size,
            Self::Path { size: s, .. } => *s = size,
        }
    }

    pub fn size(&self) -> BufferAddress {
        match self {
            Self::Root { size, .. } => *size,
            Self::Node { size, .. } => *size,
            Self::Path { size, .. } => *size,
        }
    }
}

impl HitboxNode<Self> for ToolpathTree {
    fn check_hit(&self, ray: &rether::picking::Ray) -> Option<f32> {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.check_hit(ray),
            Self::Node { bounding_box, .. } => bounding_box.check_hit(ray),
            Self::Path { path_box, .. } => path_box.check_hit(ray),
        }
    }

    fn inner_nodes(&self) -> &[Self] {
        match self {
            Self::Root { children, .. } => children,
            Self::Node { children, .. } => children,
            Self::Path { .. } => &[],
        }
    }

    fn get_min(&self) -> glam::Vec3 {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.get_min(),
            Self::Node { bounding_box, .. } => bounding_box.get_min(),
            Self::Path { path_box, .. } => path_box.get_min(),
        }
    }

    fn get_max(&self) -> glam::Vec3 {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.get_max(),
            Self::Node { bounding_box, .. } => bounding_box.get_max(),
            Self::Path { path_box, .. } => path_box.get_max(),
        }
    }
}

impl InteractiveModel for ToolpathTree {
    fn clicked(&self, _event: rether::picking::interact::ClickEvent) {
        println!("ToolpathTree: Clicked");
    }

    fn drag(&self, _event: rether::picking::interact::DragEvent) {
        println!("ToolpathTree: Dragged");
    }

    fn scroll(&self, _event: rether::picking::interact::ScrollEvent) {
        println!("ToolpathTree: Scrolled");
    }
}

impl Translate for ToolpathTree {
    fn translate(&mut self, translation: glam::Vec3) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.translate(translation);
                bounding_box.translate(translation);

                for child in children {
                    child.translate(translation);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.translate(translation);

                for child in children {
                    child.translate(translation);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.translate(translation);
            }
        }
    }
}

impl Rotate for ToolpathTree {
    fn rotate(&mut self, rotation: glam::Quat) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.rotate(rotation);
                bounding_box.rotate(rotation);

                for child in children {
                    child.rotate(rotation);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.rotate(rotation);

                for child in children {
                    child.rotate(rotation);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.rotate(rotation);
            }
        }
    }
}

impl Scale for ToolpathTree {
    fn scale(&mut self, scale: glam::Vec3) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.scale(scale);
                bounding_box.scale(scale);

                for child in children {
                    child.scale(scale);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.scale(scale);

                for child in children {
                    child.scale(scale);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.scale(scale);
            }
        }
    }
}
