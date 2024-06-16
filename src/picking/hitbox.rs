use crate::geometry::BoundingBox; // Importing the BoundingBox struct from the geometry module in the crate

use super::{queue::HitBoxQueueEntry, queue::HitboxQueue, ray::Ray}; // Importing the Ray struct from the ray module in the super namespace

// Definition of the HitBoxRoot struct with Debug and Default traits
#[derive(Debug, Default)]
pub struct HitBoxRoot {
    hitboxes: Vec<HitboxNode>, // Vector of HitboxNode
}

// Implementation of methods for HitBoxRoot
impl HitBoxRoot {
    // Method to add a hitbox to the hitboxes vector
    pub fn add_hitbox(&mut self, hitbox: HitboxNode) {
        self.hitboxes.push(hitbox);
    }
}

// Function to check if a ray hits a hitbox node, returning an optional usize
fn check_hit(hitbox: &HitboxNode, ray: &Ray) -> Option<usize> {
    let mut queue = HitboxQueue::new(); // Creating a new HitboxQueue

    let distance = ray.closest_distance_box(&hitbox.bounding_box());

    queue.push(HitBoxQueueEntry { hitbox, distance });

    while let Some(HitBoxQueueEntry { hitbox, .. }) = queue.pop() {
        match hitbox {
            // If hitbox is a ParentBox, check if the ray intersects the bounding box
            HitboxNode::ParentBox {
                inner_hitboxes,
                bounding_box,
            } => {
                if ray.intersects_box(bounding_box) {
                    // If it intersects, recursively check inner hitboxes
                    for hitbox in inner_hitboxes {
                        let distance = ray.closest_distance_box(&hitbox.bounding_box());
                        queue.push(HitBoxQueueEntry { hitbox, distance });
                    }
                }
            }
            // If hitbox is a Box, check if the ray intersects its bounding box
            HitboxNode::Box { boundind_box, id } => {
                if ray.intersects_box(boundind_box) {
                    return Some(*id);
                }
            }
        }
    }

    None
}

// Definition of the HitboxNode enum with Debug trait
#[derive(Debug)]
pub enum HitboxNode {
    // Variant for parent boxes containing other hitboxes and a bounding box
    ParentBox {
        inner_hitboxes: Vec<HitboxNode>,
        bounding_box: BoundingBox,
    },
    // Variant for individual boxes with a bounding box and an id
    Box {
        boundind_box: BoundingBox,
        id: usize,
    },
}

// Implementation of methods for HitboxNode
impl HitboxNode {
    // Constructor method for creating a parent box
    pub fn parent_box(bounding_box: BoundingBox) -> Self {
        HitboxNode::ParentBox {
            inner_hitboxes: Vec::new(),
            bounding_box,
        }
    }

    // Constructor method for creating a box with an id
    pub fn box_(bounding_box: BoundingBox, id: usize) -> Self {
        HitboxNode::Box {
            boundind_box: bounding_box,
            id,
        }
    }

    // Method to add a hitbox to a parent box
    pub fn add_hitbox(&mut self, hitbox: HitboxNode) {
        match self {
            // If the hitbox is a ParentBox, expand its bounding box and add the new hitbox
            HitboxNode::ParentBox {
                inner_hitboxes,
                bounding_box,
            } => {
                bounding_box.expand(&hitbox.bounding_box());
                inner_hitboxes.push(hitbox);
            }
            // If the hitbox is a Box, do nothing
            HitboxNode::Box { .. } => {}
        }
    }

    // Method to get the bounding box of a hitbox node
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            HitboxNode::ParentBox { bounding_box, .. } => bounding_box.clone(),
            HitboxNode::Box { boundind_box, .. } => boundind_box.clone(),
        }
    }

    // Method to get a mutable reference to the bounding box of a hitbox node
    pub fn boundind_box_mut(&mut self) -> &mut BoundingBox {
        match self {
            HitboxNode::ParentBox { bounding_box, .. } => bounding_box,
            HitboxNode::Box { boundind_box, .. } => boundind_box,
        }
    }
}

// Test function for hitbox functionality
#[test]
pub fn test_hitbox() {
    use glam::vec3;
    use glam::Vec3;

    let mut root = HitBoxRoot::default(); // Creating a default HitBoxRoot

    let box_ = HitboxNode::box_(
        BoundingBox::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0)), // Creating a bounding box with specific dimensions
        30,
    );

    root.add_hitbox(box_); // Adding the box to the root

    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 1.0, 1.0),
    };

    let hit = check_hit(&root.hitboxes[0], &ray); // Checking if the ray hits the box

    assert_eq!(hit, Some(30)); // Asserting that the hit id is 30
}

// Test function for hitbox parent functionality
#[test]
pub fn test_hitbox_parent() {
    use glam::vec3;
    use glam::Vec3; // Importing Vec3 from glam crate

    let now = std::time::Instant::now(); // Creating a new Instant

    let mut root = HitBoxRoot::default(); // Creating a default HitBoxRoot

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

    let hit = check_hit(&root.hitboxes[0], &ray); // Checking if the ray hits any of the boxes

    assert_eq!(hit, Some(30)); // Asserting that the hit id is 30

    println!("Time: {:?}", now.elapsed()); // Printing the elapsed time
    assert!(false)
}
