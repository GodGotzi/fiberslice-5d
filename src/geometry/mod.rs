use glam::{vec3, Vec3};
use mesh::WireMesh;

use crate::picking::{hitbox::Hitbox, ray::EPSILON};

pub mod mesh;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub max: Vec3,
    pub min: Vec3,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
        }
    }
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { max, min }
    }

    pub fn center(&self) -> Vec3 {
        (self.max + self.min) / 2.0
    }

    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn expand(&mut self, other: &Self) {
        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);

        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);
    }

    pub fn expand_point(&mut self, point: Vec3) {
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);

        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
    }

    pub fn contains(&self, point: Vec3) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.y <= point.y
            && point.y <= self.max.y
            && self.min.z <= point.z
            && point.z <= self.max.z
    }

    pub fn corners(&self) -> [Vec3; 8] {
        [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ]
    }

    pub fn planes(&self) -> [(Vec3, Vec3); 6] {
        [
            (Vec3::new(1.0, 0.0, 0.0), self.max),
            (Vec3::new(-1.0, 0.0, 0.0), self.min),
            (Vec3::new(0.0, 1.0, 0.0), self.max),
            (Vec3::new(0.0, -1.0, 0.0), self.min),
            (Vec3::new(0.0, 0.0, 1.0), self.max),
            (Vec3::new(0.0, 0.0, -1.0), self.min),
        ]
    }

    pub fn faces_with_edges(&self) -> [(Vec3, (Vec3, Vec3, Vec3, Vec3)); 6] {
        [
            (
                Vec3::new(1.0, 0.0, 0.0),
                (
                    Vec3::new(self.max.x, self.max.y, self.max.z),
                    Vec3::new(self.max.x, self.min.y, self.max.z),
                    Vec3::new(self.max.x, self.min.y, self.min.z),
                    Vec3::new(self.max.x, self.max.y, self.min.z),
                ),
            ),
            (
                Vec3::new(-1.0, 0.0, 0.0),
                (
                    Vec3::new(self.min.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.min.z),
                    Vec3::new(self.min.x, self.max.y, self.min.z),
                ),
            ),
            (
                Vec3::new(0.0, 1.0, 0.0),
                (
                    Vec3::new(self.max.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.max.y, self.min.z),
                    Vec3::new(self.max.x, self.max.y, self.min.z),
                ),
            ),
            (
                Vec3::new(0.0, -1.0, 0.0),
                (
                    Vec3::new(self.max.x, self.min.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.min.z),
                    Vec3::new(self.max.x, self.min.y, self.min.z),
                ),
            ),
            (
                Vec3::new(0.0, 0.0, 1.0),
                (
                    Vec3::new(self.max.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.max.z),
                    Vec3::new(self.max.x, self.min.y, self.max.z),
                ),
            ),
            (
                Vec3::new(0.0, 0.0, -1.0),
                (
                    Vec3::new(self.max.x, self.max.y, self.min.z),
                    Vec3::new(self.min.x, self.max.y, self.min.z),
                    Vec3::new(self.min.x, self.min.y, self.min.z),
                    Vec3::new(self.max.x, self.min.y, self.min.z),
                ),
            ),
        ]
    }
}

impl Hitbox for BoundingBox {
    fn check_hit(&self, ray: &crate::picking::ray::Ray) -> Option<f32> {
        if self.contains(ray.origin) {
            println!("PickingAdapter: Ray origin is inside the bounding box");
            return Some(0.0);
        }

        println!("PickingAdapter: Ray origin is outside the bounding box");

        let mut min = None;

        for (plane_dir, (a, b, c, d)) in self.faces_with_edges() {
            let intersection = ray.intersection_plane(plane_dir, a);

            let max_face = a.max(b).max(c).max(d);
            let min_face = a.min(b).min(c).min(d);

            // check if the intersection point is inside the face with epsilon
            if (max_face.x + EPSILON) >= intersection.x
                && intersection.x >= (min_face.x - EPSILON)
                && (max_face.y + EPSILON) >= intersection.y
                && intersection.y >= (min_face.y - EPSILON)
                && (max_face.z + EPSILON) >= intersection.z
                && intersection.z >= (min_face.z - EPSILON)
            {
                let distance = (intersection - ray.origin).length();
                if min.unwrap_or(f32::MAX) > distance || min.is_none() {
                    min = Some(distance);
                }
            }
        }

        min
    }

    fn expand(&mut self, _box: &Box<dyn Hitbox>) {
        self.min = self.min.min(_box.min());
        self.max = self.max.max(_box.max());
    }

    fn min(&self) -> Vec3 {
        self.min
    }

    fn max(&self) -> Vec3 {
        self.max
    }
}

pub struct Quad {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct SlimBox {
    front: Quad,
    back: Quad,
    left: Quad,
    right: Quad,
    top: Quad,
    bottom: Quad,
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
