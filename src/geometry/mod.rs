use glam::{vec3, Vec3};
use mesh::WireMesh;

pub mod r#box;
pub mod mesh;

pub use r#box::BoundingBox;

use crate::{picking::hitbox::Hitbox, prelude::SharedMut};

#[derive(Debug, Clone, Copy)]
pub struct QuadFace {
    pub normal: Vec3,
    pub min: Vec3,
    pub max: Vec3,
}

impl Hitbox for QuadFace {
    fn check_hit(&self, ray: &crate::picking::ray::Ray) -> Option<f32> {
        let intersection = ray.intersection_plane(self.normal, self.min);

        const EPSILON: f32 = 0.0001;

        // check if the intersection point is inside the face with epsilon
        if (self.max.x + EPSILON) >= intersection.x
            && intersection.x >= (self.min.x - EPSILON)
            && (self.max.y + EPSILON) >= intersection.y
            && intersection.y >= (self.min.y - EPSILON)
            && (self.max.z + EPSILON) >= intersection.z
            && intersection.z >= (self.min.z - EPSILON)
        {
            let distance = (intersection - ray.origin).length();
            Some(distance)
        } else {
            None
        }
    }

    fn expand(&mut self, _box: &SharedMut<Box<dyn Hitbox>>) {
        // Not expandable
        // TODO either figure out how to expand this or remove this method for this type or make it clear that this is not expandable
    }

    fn min(&self) -> Vec3 {
        self.min
    }

    fn max(&self) -> Vec3 {
        self.max
    }
}

pub struct SelectBox {
    box_: BoundingBox,
}

impl From<BoundingBox> for SelectBox {
    fn from(box_: BoundingBox) -> Self {
        // box_.expand_point(box_.max + Vec3::new(2.0, 2.0, 2.0));
        // box_.expand_point(box_.min + Vec3::new(-2.0, -2.0, -2.0));

        Self { box_ }
    }
}

impl SelectBox {
    pub const fn traingle_vertex_count() -> usize {
        72
    }

    pub const fn wire_vertex_count() -> usize {
        28
    }
}

impl crate::geometry::mesh::Mesh<72> for SelectBox {
    fn to_triangle_vertices(&self) -> [glam::Vec3; 72] {
        let corner_expansion = 0.2
            * (self.box_.min.x - self.box_.max.x)
                .abs()
                .min((self.box_.min.y - self.box_.max.y).abs())
                .min((self.box_.min.z - self.box_.max.z).abs());

        [
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.min.y,
                self.box_.min.z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.min.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion,
            ),
            vec3(
                self.box_.min.x + corner_expansion,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.min.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion,
            ),
            vec3(
                self.box_.max.x - corner_expansion,
                self.box_.min.y,
                self.box_.min.z,
            ),
        ]
    }
}

impl WireMesh<28> for SelectBox {
    fn to_wire_vertices(&self) -> [glam::Vec3; 28] {
        [
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
        ]
    }
}
