use glam::{vec3, Vec3, Vec4};
use mesh::{construct_triangle, construct_triangle_vertices, construct_wire_vertices, WireMesh};

pub mod r#box;
pub mod mesh;

pub use r#box::BoundingHitbox;

use crate::{picking::hitbox::Hitbox, prelude::SharedMut, render::vertex::Vertex};

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
        panic!("QuadFace does not have an expand method");
    }

    fn set_enabled(&mut self, _enabled: bool) {
        panic!("QuadFace does not have an enabled method");
    }

    fn enabled(&self) -> bool {
        panic!("QuadFace does not have an enabled method");
    }

    fn min(&self) -> Vec3 {
        self.min
    }

    fn max(&self) -> Vec3 {
        self.max
    }
}

pub struct SelectBox {
    box_: BoundingHitbox,
}

impl From<BoundingHitbox> for SelectBox {
    fn from(box_: BoundingHitbox) -> Self {
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
    fn to_triangle_vertices(&self) -> [Vertex; 72] {
        let corner_expansion = 0.2
            * (self.box_.min.x - self.box_.max.x)
                .abs()
                .min((self.box_.min.y - self.box_.max.y).abs())
                .min((self.box_.min.z - self.box_.max.z).abs());

        construct_triangle_vertices(
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
            ],
            Vec4::new(0.0, 1.0, 0.0, 1.0),
        )
    }
}

impl WireMesh<28> for SelectBox {
    fn to_wire_vertices(&self) -> [Vertex; 28] {
        construct_wire_vertices(
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
            ],
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}
