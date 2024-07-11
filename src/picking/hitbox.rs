use glam::Vec3;
use log::info;

use crate::{
    model::transform::{Rotate, Scale, Translate},
    prelude::SharedMut,
}; // Importing the BoundingBox struct from the geometry module in the crate

use super::{
    interactive::Pickable,
    queue::{HitBoxQueueEntry, HitboxQueue},
    ray::Ray,
};

pub trait Hitbox: std::fmt::Debug + Send + Sync + Translate + Rotate + Scale {
    fn check_hit(&self, ray: &Ray) -> Option<f32>;
    fn expand(&mut self, _box: &dyn Hitbox);
    fn set_enabled(&mut self, enabled: bool);
    fn enabled(&self) -> bool;
    fn min(&self) -> Vec3;
    fn max(&self) -> Vec3;
}

pub type PickContext = SharedMut<Box<dyn Pickable>>;

// Importing the Ray struct from the ray module in the super namespace
// Function to check if a ray hits a hitbox node, returning an optional usize

// Definition of the HitboxNode enum with Debug trait
#[derive(Debug, Clone)]
pub enum HitboxNode {
    // Variant for parent boxes containing other hitboxes and a bounding box
    Root {
        inner_hitboxes: Vec<HitboxNode>,
    },
    ParentBox {
        inner_hitboxes: Vec<HitboxNode>,
        ctx: PickContext,
    },
    // Variant for individual boxes with a bounding box and an id
    Box {
        ctx: PickContext,
    },
}

// Implementation of methods for HitboxNode
impl HitboxNode {
    pub fn root() -> Self {
        HitboxNode::Root {
            inner_hitboxes: Vec::new(),
        }
    }

    pub fn parent_box(ctx: PickContext, inner_hitboxes: Vec<HitboxNode>) -> Self {
        HitboxNode::ParentBox {
            inner_hitboxes,
            ctx,
        }
    }

    fn hitbox(&self) -> &dyn Hitbox {
        match self {
            HitboxNode::ParentBox { ctx, .. } => ctx,
            HitboxNode::Box { ctx } => ctx,
            HitboxNode::Root { .. } => panic!("Root does not have a hitbox"),
        }
    }

    pub fn check_hit(&self, ray: &Ray) -> Option<&PickContext> {
        let mut queue = HitboxQueue::new(); // Creating a new HitboxQueue

        if let HitboxNode::Root { inner_hitboxes } = self {
            for hitbox in inner_hitboxes {
                let distance = hitbox.hitbox().check_hit(ray);
                if let Some(distance) = distance {
                    queue.push(HitBoxQueueEntry { hitbox, distance });
                }
            }
        }

        while let Some(HitBoxQueueEntry { hitbox, .. }) = queue.pop() {
            match hitbox {
                // If hitbox is a ParentBox, check if the ray intersects the bounding box
                HitboxNode::ParentBox { inner_hitboxes, .. }
                | HitboxNode::Root { inner_hitboxes, .. } => {
                    // If it intersects, recursively check inner hitboxes
                    for hitbox in inner_hitboxes {
                        let distance = hitbox.hitbox().check_hit(ray);
                        if let Some(distance) = distance {
                            queue.push(HitBoxQueueEntry { hitbox, distance });
                        }
                    }
                }
                // If hitbox is a Box, check if the ray intersects its bounding box
                HitboxNode::Box { ctx, .. } => {
                    return Some(ctx);
                }
            }
        }

        println!("Ray does not intersect any hitbox");

        None
    }

    pub fn add_node(&mut self, node: HitboxNode) {
        match self {
            HitboxNode::Root { inner_hitboxes, .. } => {
                inner_hitboxes.push(node);
            }
            HitboxNode::ParentBox {
                inner_hitboxes,
                ctx: handle,
                ..
            } => {
                handle.expand(node.hitbox());
                inner_hitboxes.push(node);
            }
            HitboxNode::Box { .. } => {
                panic!("Cannot add node to a box");
            }
        }
    }
}

/*
// Test function for hitbox functionality
#[test]
pub fn test_hitbox() {
    use glam::vec3;
    use glam::Vec3;

    let mut root = HitboxNode::parent_box(BoundingBox::default()); // Creating a default HitBoxRoot

    let box_ = HitboxNode::box_(
        BoundingBox::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0)), // Creating a bounding box with specific dimensions
        Arc::new(0),
    );

    root.add_hitbox(box_); // Adding the box to the root

    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 1.0, 1.0),
    };

    let hit = root.check_hit(&ray); // Checking if the ray hits the box

    assert_eq!(hit, Some(30)); // Asserting that the hit id is 30
}

// Test function for hitbox parent functionality
#[test]
pub fn test_hitbox_parent() {
    use glam::vec3;
    use glam::Vec3; // Importing Vec3 from glam crate

    let mut root = HitboxNode::parent_box(BoundingBox::default()); // Creating a default HitBoxRoot

    let mut parent =
        HitboxNode::parent_box(BoundingBox::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0))); // Creating a parent box with specific dimensions

    let box_ = HitboxNode::box_(
        BoundingBox::new(vec3(0.0, 0.0, 0.0), vec3(0.5, 0.5, 0.5)), // Creating a smaller bounding box
        30,
    );

    parent.add_hitbox(box_); // Adding the smaller box to the parent box

    root.add_hitbox(parent); // Adding the parent box to the root

    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 1.0, 1.0),
    };

    let hit = root.check_hit(&ray); // Checking if the ray hits any of the boxes

    assert_eq!(hit, Some(30)); // Asserting that the hit id is 30
}
*/
