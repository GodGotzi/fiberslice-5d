use parking_lot::RwLock;
use wgpu::BufferAddress;

use crate::{
    geometry::BoundingBox,
    picking::{
        hitbox::{Hitbox, HitboxNode},
        interact::InteractiveModel,
    },
    prelude::LockModel,
    render::model::{Model, Rotate, RotateMut, Scale, ScaleMut, Translate, TranslateMut},
    render::Renderable,
};

use super::{mesh::PathHitbox, vertex::ToolpathVertex};

#[derive(Debug)]
pub enum ToolpathTree {
    Root {
        model: LockModel<ToolpathVertex>,
        bounding_box: RwLock<BoundingBox>,
        children: Vec<Self>,
        size: BufferAddress,
    },
    Node {
        offset: BufferAddress,
        size: BufferAddress,
        bounding_box: RwLock<BoundingBox>,
        children: Vec<Self>,
    },
    Path {
        offset: BufferAddress,
        size: BufferAddress,
        path_box: RwLock<Box<PathHitbox>>,
    },
}

impl ToolpathTree {
    pub fn create_root() -> Self {
        Self::Root {
            model: LockModel::new(Model::create()),
            children: Vec::new(),
            bounding_box: RwLock::new(BoundingBox::default()),
            size: 0,
        }
    }

    pub fn create_root_with_children(
        bounding_box: BoundingBox,
        children: Vec<Self>,
        size: BufferAddress,
    ) -> Self {
        Self::Root {
            model: LockModel::new(Model::create()),
            children,
            bounding_box: RwLock::new(bounding_box),
            size,
        }
    }

    pub fn create_node(offset: BufferAddress, size: BufferAddress) -> Self {
        Self::Node {
            offset,
            size,
            children: Vec::new(),
            bounding_box: RwLock::new(BoundingBox::default()),
        }
    }

    pub fn create_node_with_children(
        children: Vec<Self>,
        offset: BufferAddress,
        size: BufferAddress,
    ) -> Self {
        Self::Node {
            offset,
            size,
            children,
            bounding_box: RwLock::new(BoundingBox::default()),
        }
    }

    pub fn create_path(path_box: PathHitbox, offset: BufferAddress, size: BufferAddress) -> Self {
        Self::Path {
            offset,
            size,
            path_box: RwLock::new(Box::new(path_box)),
        }
    }

    pub fn push_node(&mut self, node: Self) {
        match self {
            Self::Root {
                children,
                bounding_box,
                size,
                ..
            } => {
                *size += node.size();
                bounding_box.get_mut().expand_point(node.get_min());
                bounding_box.get_mut().expand_point(node.get_max());
                children.push(node);
            }
            Self::Node {
                children,
                bounding_box,
                size,
                ..
            } => {
                *size += node.size();
                bounding_box.get_mut().expand_point(node.get_min());
                bounding_box.get_mut().expand_point(node.get_max());
                children.push(node);
            }
            Self::Path { .. } => panic!("Cannot push node to path"),
        }
    }

    pub fn push_path(&mut self, path: Self) {
        match self {
            Self::Root {
                children,
                bounding_box,
                size,
                ..
            } => {
                *size += path.size();
                bounding_box.get_mut().expand_point(path.get_min());
                bounding_box.get_mut().expand_point(path.get_max());
                children.push(path);
            }
            Self::Node {
                children,
                bounding_box,
                size,
                ..
            } => {
                *size += path.size();
                bounding_box.get_mut().expand_point(path.get_min());
                bounding_box.get_mut().expand_point(path.get_max());
                children.push(path);
            }
            Self::Path { .. } => panic!("Cannot push path to path"),
        }
    }

    pub fn update_offset(&mut self, offset: BufferAddress) {
        match self {
            Self::Root { children, .. } => {
                let mut current_offset = offset;
                for child in children {
                    child.update_offset(current_offset);
                    current_offset += child.size();
                }
            }
            Self::Node { children, .. } => {
                let mut current_offset = offset;
                for child in children {
                    child.update_offset(current_offset);
                    current_offset += child.size();
                }
            }
            Self::Path { offset: o, .. } => *o = offset,
        }
    }

    pub fn size(&self) -> BufferAddress {
        match self {
            Self::Root { size, .. } => *size,
            Self::Node { size, .. } => *size,
            Self::Path { size, .. } => *size,
        }
    }

    pub fn awaken(&mut self, data: &[ToolpathVertex]) {
        match self {
            Self::Root { model, .. } => model.write().awaken(data),
            Self::Node { .. } => panic!("Cannot awaken node"),
            Self::Path { .. } => panic!("Cannot awaken path"),
        }
    }
}

impl Renderable for ToolpathTree {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        match self {
            Self::Root { model, .. } => model.render(render_pass),
            Self::Node { .. } => panic!("Cannot render node"),
            Self::Path { .. } => panic!("Cannot render path"),
        }
    }
}

impl HitboxNode<Self> for ToolpathTree {
    fn check_hit(&self, ray: &crate::picking::Ray) -> Option<f32> {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.read().check_hit(ray),
            Self::Node { bounding_box, .. } => bounding_box.read().check_hit(ray),
            Self::Path { path_box, .. } => path_box.read().check_hit(ray),
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
            Self::Root { bounding_box, .. } => bounding_box.read().get_min(),
            Self::Node { bounding_box, .. } => bounding_box.read().get_min(),
            Self::Path { path_box, .. } => path_box.read().get_min(),
        }
    }

    fn get_max(&self) -> glam::Vec3 {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.read().get_max(),
            Self::Node { bounding_box, .. } => bounding_box.read().get_max(),
            Self::Path { path_box, .. } => path_box.read().get_max(),
        }
    }
}

impl InteractiveModel for ToolpathTree {
    fn clicked(&self, _event: crate::picking::interact::ClickEvent) {
        println!("ToolpathTree: Clicked");
    }

    fn drag(&self, _event: crate::picking::interact::DragEvent) {
        println!("ToolpathTree: Dragged");
    }

    fn scroll(&self, _event: crate::picking::interact::ScrollEvent) {
        println!("ToolpathTree: Scrolled");
    }
}

impl Translate for ToolpathTree {
    fn translate(&self, translation: glam::Vec3) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.write().translate(translation);
                bounding_box.write().translate(translation);

                for child in children {
                    child.translate(translation);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.write().translate(translation);

                for child in children {
                    child.translate(translation);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.write().translate(translation);
            }
        }
    }
}

impl Rotate for ToolpathTree {
    fn rotate(&self, rotation: glam::Quat) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.write().rotate(rotation);
                bounding_box.write().rotate(rotation);

                for child in children {
                    child.rotate(rotation);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.write().rotate(rotation);

                for child in children {
                    child.rotate(rotation);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.write().rotate(rotation);
            }
        }
    }
}

impl Scale for ToolpathTree {
    fn scale(&self, scale: glam::Vec3) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.write().scale(scale);
                bounding_box.write().scale(scale);

                for child in children {
                    child.scale(scale);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.write().scale(scale);

                for child in children {
                    child.scale(scale);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.write().scale(scale);
            }
        }
    }
}

impl TranslateMut for ToolpathTree {
    fn translate(&mut self, translation: glam::Vec3) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.get_mut().translate(translation);
                bounding_box.get_mut().translate(translation);

                for child in children {
                    child.translate(translation);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.get_mut().translate(translation);

                for child in children {
                    child.translate(translation);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.get_mut().translate(translation);
            }
        }
    }
}

impl RotateMut for ToolpathTree {
    fn rotate(&mut self, rotation: glam::Quat) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.get_mut().rotate(rotation);
                bounding_box.get_mut().rotate(rotation);

                for child in children {
                    child.rotate(rotation);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.get_mut().rotate(rotation);

                for child in children {
                    child.rotate(rotation);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.get_mut().rotate(rotation);
            }
        }
    }
}

impl ScaleMut for ToolpathTree {
    fn scale(&mut self, scale: glam::Vec3) {
        match self {
            ToolpathTree::Root {
                model,
                children,
                bounding_box,
                ..
            } => {
                model.get_mut().scale(scale);
                bounding_box.get_mut().scale(scale);

                for child in children {
                    child.scale(scale);
                }
            }
            ToolpathTree::Node {
                children,
                bounding_box,
                ..
            } => {
                bounding_box.get_mut().scale(scale);

                for child in children {
                    child.scale(scale);
                }
            }
            ToolpathTree::Path { path_box, .. } => {
                path_box.get_mut().scale(scale);
            }
        }
    }
}
