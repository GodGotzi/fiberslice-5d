use glam::Vec3;

use crate::{prelude::SharedMut, render::model::MeshHandle}; // Importing the BoundingBox struct from the geometry module in the crate

use super::{
    queue::{HitBoxQueueEntry, HitboxQueue},
    ray::Ray,
};

pub trait Hitbox: std::fmt::Debug + Send + Sync {
    fn check_hit(&self, ray: &Ray) -> Option<f32>;
    fn expand(&mut self, _box: &SharedMut<Box<dyn Hitbox>>);
    fn set_enabled(&mut self, enabled: bool);
    fn enabled(&self) -> bool;
    fn min(&self) -> Vec3;
    fn max(&self) -> Vec3;
}

// Importing the Ray struct from the ray module in the super namespace
// Function to check if a ray hits a hitbox node, returning an optional usize

// Definition of the HitboxNode enum with Debug trait
#[derive(Debug)]
pub enum HitboxNode {
    // Variant for parent boxes containing other hitboxes and a bounding box
    ParentBox {
        inner_hitboxes: Vec<HitboxNode>,
        _box: SharedMut<Box<dyn Hitbox>>,
    },
    // Variant for individual boxes with a bounding box and an id
    Box {
        _box: SharedMut<Box<dyn Hitbox>>,
        interactive_mesh: MeshHandle,
    },
}

// Implementation of methods for HitboxNode
impl HitboxNode {
    // Constructor method for creating a parent box
    pub fn parent_box(_box: SharedMut<Box<dyn Hitbox>>) -> Self {
        HitboxNode::ParentBox {
            inner_hitboxes: Vec::new(),
            _box,
        }
    }

    // Constructor method for creating a box with an id
    pub fn box_(_box: SharedMut<Box<dyn Hitbox>>, mesh: MeshHandle) -> Self {
        HitboxNode::Box {
            interactive_mesh: mesh,
            _box,
        }
    }

    // Method to add a hitbox to a parent box
    pub fn add_hitbox(&mut self, hitbox: HitboxNode) {
        match self {
            // If the hitbox is a ParentBox, expand its bounding box and add the new hitbox
            HitboxNode::ParentBox {
                inner_hitboxes,
                _box,
            } => {
                _box.write().expand(hitbox._box());
                inner_hitboxes.push(hitbox);
            }
            // If the hitbox is a Box, do nothing
            HitboxNode::Box { .. } => {}
        }
    }

    // Method to get the bounding box of a hitbox node
    pub fn _box(&self) -> &SharedMut<Box<dyn Hitbox>> {
        match self {
            HitboxNode::ParentBox { _box, .. } => _box,
            HitboxNode::Box { _box, .. } => _box,
        }
    }

    pub fn check_hit(&self, ray: &Ray) -> Option<&MeshHandle> {
        let mut queue = HitboxQueue::new(); // Creating a new HitboxQueue

        let distance = self._box().read().check_hit(ray);
        if let Some(distance) = distance {
            queue.push(HitBoxQueueEntry {
                hitbox: self,
                distance,
            });
        }

        while let Some(HitBoxQueueEntry { hitbox, .. }) = queue.pop() {
            match hitbox {
                // If hitbox is a ParentBox, check if the ray intersects the bounding box
                HitboxNode::ParentBox { inner_hitboxes, .. } => {
                    // If it intersects, recursively check inner hitboxes
                    for hitbox in inner_hitboxes {
                        let distance = self._box().read().check_hit(ray);
                        if let Some(distance) = distance {
                            queue.push(HitBoxQueueEntry { hitbox, distance });
                        }
                    }
                }
                // If hitbox is a Box, check if the ray intersects its bounding box
                HitboxNode::Box {
                    interactive_mesh, ..
                } => {
                    return Some(interactive_mesh);
                }
            }
        }

        println!("Ray does not intersect any hitbox");

        None
    }
}

impl From<MeshHandle> for HitboxNode {
    fn from(mesh: MeshHandle) -> Self {
        match &mesh {
            MeshHandle::Static { .. } => panic!("Static mesh cannot be converted to hitbox"),
            MeshHandle::Interactive { raw_box, .. } => HitboxNode::box_(raw_box.clone(), mesh),
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
