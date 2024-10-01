use parking_lot::RwLock;
use rether::{
    alloc::DynamicAllocHandle,
    model::{
        geometry::Geometry, BufferLocation, Expandable, Model, ModelState, RotateModel, ScaleModel,
        TranslateModel, TreeModel,
    },
    picking::{interact::InteractiveModel, Hitbox, HitboxNode},
    vertex::Vertex,
    Rotate, Scale, Translate,
};

use crate::geometry::BoundingBox;

use super::mesh::PathHitbox;

#[derive(Debug)]
pub enum ToolpathTree {
    Root {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
        bounding_box: RwLock<BoundingBox>,
    },
    Node {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
        bounding_box: RwLock<BoundingBox>,
    },
    Path {
        model: TreeModel<Self, Vertex, DynamicAllocHandle<Vertex>>,
        path_box: RwLock<Box<PathHitbox>>,
    },
}

impl ToolpathTree {
    pub fn create_root<M: Into<ModelState<Vertex, DynamicAllocHandle<Vertex>>>>(
        bounding_box: BoundingBox,
        geometry: M,
    ) -> Self {
        Self::Root {
            model: TreeModel::create_root(geometry),
            bounding_box: RwLock::new(bounding_box),
        }
    }

    pub fn create_root_with_models<M: Into<ModelState<Vertex, DynamicAllocHandle<Vertex>>>>(
        bounding_box: BoundingBox,
        geometry: M,
        models: Vec<ToolpathTree>,
    ) -> Self {
        Self::Root {
            model: TreeModel::create_root_with_models(geometry, models),
            bounding_box: RwLock::new(bounding_box),
        }
    }

    pub fn create_node(bounding_box: BoundingBox, location: BufferLocation) -> Self {
        Self::Node {
            model: TreeModel::create_node(location),
            bounding_box: RwLock::new(bounding_box),
        }
    }

    pub fn create_node_with_models(
        bounding_box: BoundingBox,
        location: BufferLocation,
        models: Vec<ToolpathTree>,
    ) -> Self {
        Self::Node {
            model: TreeModel::create_node_with_models(location, models),
            bounding_box: RwLock::new(bounding_box),
        }
    }

    pub fn create_path(path_box: PathHitbox, location: BufferLocation) -> Self {
        Self::Path {
            model: TreeModel::create_node(location),
            path_box: RwLock::new(Box::new(path_box)),
        }
    }

    pub fn add_child(&mut self, child: Self) {
        let other_bounding_box = match &child {
            Self::Root { bounding_box, .. } => *bounding_box.read(),
            Self::Node { bounding_box, .. } => *bounding_box.read(),
            Self::Path { path_box, .. } => {
                let min = path_box.read().get_min();
                let max = path_box.read().get_max();
                BoundingBox::new(min, max)
            }
        };

        let model = match self {
            Self::Root {
                model,
                bounding_box,
            } => {
                bounding_box.get_mut().expand(&other_bounding_box);
                model
            }
            Self::Node {
                model,
                bounding_box,
            } => {
                bounding_box.get_mut().expand(&other_bounding_box);
                model
            }
            Self::Path { .. } => {
                panic!("Cannot add child to path");
            }
        };

        match model {
            TreeModel::Root {
                state, sub_handles, ..
            } => {
                let other_model = match child {
                    Self::Root { model, .. } => model,
                    Self::Node { model, .. } => model,
                    Self::Path { model, .. } => model,
                };

                let node = match other_model {
                    TreeModel::Root {
                        state: mut other_state,
                        mut sub_handles,
                        ..
                    } => {
                        let (offset, size) = match state.get_mut() {
                            ModelState::Dormant(geometry) => {
                                match other_state.get_mut() {
                                    ModelState::Dormant(other_geometry) => {
                                        let offset = geometry.data_len();

                                        geometry.expand(other_geometry);

                                        (offset, other_geometry.data_len())
                                    }
                                    ModelState::DormantIndexed(_) => {
                                        panic!("Cannot expand a dormant geometry with an indexed geometry");
                                    }
                                    _ => panic!("Cannot expand an alive or dead handle"),
                                }
                            }
                            ModelState::DormantIndexed(geometry) => {
                                match other_state.get_mut() {
                                    ModelState::Dormant(_) => {
                                        panic!("Cannot expand a dormant geometry with an indexed geometry");
                                    }
                                    ModelState::DormantIndexed(other_geometry) => {
                                        let offset = geometry.data_len();

                                        geometry.expand(other_geometry);

                                        (offset, other_geometry.data_len())
                                    }
                                    _ => panic!("Cannot expand an alive or dead handle"),
                                }
                            }
                            _ => panic!("Cannot expand an alive or dead handle"),
                        };

                        for other_child in sub_handles.iter_mut() {
                            let model = match other_child {
                                Self::Root { model, .. } => model,
                                Self::Node { model, .. } => model,
                                Self::Path { model, .. } => model,
                            };

                            let location = match model {
                                TreeModel::Root { .. } => {
                                    panic!("Cannot add root as child to node")
                                }
                                TreeModel::Node { location, .. } => location,
                                TreeModel::Leaf { location } => location,
                            };

                            location.offset += offset;
                        }

                        Self::create_node_with_models(
                            other_bounding_box,
                            BufferLocation { offset, size },
                            sub_handles,
                        )
                    }
                    TreeModel::Node { .. } | TreeModel::Leaf { .. } => {
                        panic!(
                            "Cannot add node or leaf as child to root cause geometry is not known"
                        )
                    }
                };

                sub_handles.push(node);
            }
            TreeModel::Node {
                location,
                sub_handles,
            } => {
                let other_model = match child {
                    Self::Root { model, .. } => model,
                    Self::Node { model, .. } => model,
                    Self::Path { model, .. } => model,
                };

                let node = match other_model {
                    TreeModel::Root { .. } => panic!("Cannot add root as child to node"),
                    TreeModel::Node {
                        location: mut other_location,
                        mut sub_handles,
                    } => {
                        other_location.offset = location.size;
                        location.size += location.size;

                        for other_child in sub_handles.iter_mut() {
                            let model = match other_child {
                                Self::Root { model, .. } => model,
                                Self::Node { model, .. } => model,
                                Self::Path { model, .. } => model,
                            };

                            let location = match model {
                                TreeModel::Root { .. } => {
                                    panic!("Cannot add root as child to node")
                                }
                                TreeModel::Node { location, .. } => location,
                                TreeModel::Leaf { location } => location,
                            };

                            location.offset += other_location.size;
                        }

                        Self::create_node_with_models(
                            other_bounding_box,
                            other_location,
                            sub_handles,
                        )
                    }
                    TreeModel::Leaf {
                        location: mut other_location,
                    } => {
                        other_location.offset = location.size;
                        location.size += other_location.size;

                        Self::create_node_with_models(
                            other_bounding_box,
                            other_location,
                            Vec::new(),
                        )
                    }
                };

                sub_handles.push(node);
            }
            TreeModel::Leaf { .. } => panic!("Cannot add child to leaf"),
        }
    }
}

impl Model<Vertex, DynamicAllocHandle<Vertex>> for ToolpathTree {
    fn wake(&self, handle: std::sync::Arc<DynamicAllocHandle<Vertex>>) {
        match self {
            Self::Root { model, .. } => model.wake(handle),
            Self::Node { model, .. } => model.wake(handle),
            Self::Path { model, .. } => model.wake(handle),
        }
    }

    fn transform(&self) -> rether::Transform {
        match self {
            Self::Root { model, .. } => model.transform(),
            Self::Node { model, .. } => model.transform(),
            Self::Path { model, .. } => model.transform(),
        }
    }

    fn state(&self) -> &parking_lot::RwLock<ModelState<Vertex, DynamicAllocHandle<Vertex>>> {
        match self {
            Self::Root { model, .. } => model.state(),
            Self::Node { model, .. } => model.state(),
            Self::Path { model, .. } => model.state(),
        }
    }

    fn destroy(&self) {
        match self {
            Self::Root { model, .. } => model.destroy(),
            Self::Node { model, .. } => model.destroy(),
            Self::Path { model, .. } => model.destroy(),
        }
    }

    fn is_destroyed(&self) -> bool {
        match self {
            Self::Root { model, .. } => model.is_destroyed(),
            Self::Node { model, .. } => model.is_destroyed(),
            Self::Path { model, .. } => model.is_destroyed(),
        }
    }
}

impl HitboxNode<Self> for ToolpathTree {
    fn check_hit(&self, ray: &rether::picking::Ray) -> Option<f32> {
        match self {
            Self::Root { bounding_box, .. } => bounding_box.read().check_hit(ray),
            Self::Node { bounding_box, .. } => bounding_box.read().check_hit(ray),
            Self::Path { path_box, .. } => path_box.read().check_hit(ray),
        }
    }

    fn inner_nodes(&self) -> &[Self] {
        match self {
            Self::Root { model, .. } => model.sub_handles().expect("Root should have children"),
            Self::Node { model, .. } => model.sub_handles().expect("Node should have children"),
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

impl TranslateModel for ToolpathTree {
    fn translate(&self, translation: glam::Vec3) {
        match self {
            Self::Root {
                model,
                bounding_box,
            } => {
                model.translate(translation);
                bounding_box.write().translate(translation);
            }
            Self::Node {
                model,
                bounding_box,
            } => {
                model.translate(translation);
                bounding_box.write().translate(translation);
            }
            Self::Path { model, path_box } => {
                model.translate(translation);
                path_box.write().translate(translation);
            }
        }
    }
}

impl RotateModel for ToolpathTree {
    fn rotate(&self, rotation: glam::Quat, _center: Option<glam::Vec3>) {
        match self {
            Self::Root {
                model,
                bounding_box,
            } => {
                let center = bounding_box.read().center();

                model.rotate(rotation, Some(center));
                bounding_box.write().rotate(rotation, center);
            }
            Self::Node {
                model,
                bounding_box,
            } => {
                let center = bounding_box.read().center();

                model.rotate(rotation, Some(center));
                bounding_box.write().rotate(rotation, center);
            }
            Self::Path { model, path_box } => {
                let min = path_box.read().get_min();
                let max = path_box.read().get_max();

                let center = (min + max) / 2.0;

                model.rotate(rotation, Some(center));
                path_box.write().rotate(rotation, center);
            }
        }
    }
}

impl ScaleModel for ToolpathTree {
    fn scale(&self, scale: glam::Vec3, _center: Option<glam::Vec3>) {
        match self {
            Self::Root {
                model,
                bounding_box,
            } => {
                let center = bounding_box.read().center();

                model.scale(scale, Some(center));
                bounding_box.write().scale(scale);
            }
            Self::Node {
                model,
                bounding_box,
            } => {
                let center = bounding_box.read().center();

                model.scale(scale, Some(center));
                bounding_box.write().scale(scale);
            }
            Self::Path { model, path_box } => {
                let min = path_box.read().get_min();
                let max = path_box.read().get_max();

                let center = (min + max) / 2.0;

                model.scale(scale, Some(center));
                path_box.write().scale(scale);
            }
        }
    }
}
