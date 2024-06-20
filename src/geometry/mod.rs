use glam::{vec3, Vec3};

use crate::model::mesh::Mesh;

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

    pub fn faces_with_edges(&self) -> [(Vec3, Vec3, (Vec3, Vec3, Vec3, Vec3)); 6] {
        [
            (
                Vec3::new(1.0, 0.0, 0.0),
                self.max,
                (
                    Vec3::new(self.max.x, self.max.y, self.max.z),
                    Vec3::new(self.max.x, self.min.y, self.max.z),
                    Vec3::new(self.max.x, self.min.y, self.min.z),
                    Vec3::new(self.max.x, self.max.y, self.min.z),
                ),
            ),
            (
                Vec3::new(-1.0, 0.0, 0.0),
                self.min,
                (
                    Vec3::new(self.min.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.min.z),
                    Vec3::new(self.min.x, self.max.y, self.min.z),
                ),
            ),
            (
                Vec3::new(0.0, 1.0, 0.0),
                self.max,
                (
                    Vec3::new(self.max.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.max.y, self.min.z),
                    Vec3::new(self.max.x, self.max.y, self.min.z),
                ),
            ),
            (
                Vec3::new(0.0, -1.0, 0.0),
                self.min,
                (
                    Vec3::new(self.max.x, self.min.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.min.z),
                    Vec3::new(self.max.x, self.min.y, self.min.z),
                ),
            ),
            (
                Vec3::new(0.0, 0.0, 1.0),
                self.max,
                (
                    Vec3::new(self.max.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.max.y, self.max.z),
                    Vec3::new(self.min.x, self.min.y, self.max.z),
                    Vec3::new(self.max.x, self.min.y, self.max.z),
                ),
            ),
            (
                Vec3::new(0.0, 0.0, -1.0),
                self.min,
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

pub struct SelectBox {
    box_: BoundingBox,
}

impl From<BoundingBox> for SelectBox {
    fn from(mut box_: BoundingBox) -> Self {
        box_.expand_point(box_.max + Vec3::new(2.0, 2.0, 2.0));
        box_.expand_point(box_.min + Vec3::new(-2.0, -2.0, -2.0));

        Self { box_ }
    }
}

impl Mesh for SelectBox {
    fn to_vertices(&self) -> Vec<Vec3> {
        let corner_expansion_x = 0.1 * (self.box_.min.x - self.box_.max.x).abs();
        let corner_expansion_y = 0.1 * (self.box_.min.y - self.box_.max.y).abs();
        let corner_expansion_z = 0.1 * (self.box_.min.z - self.box_.max.z).abs();

        vec![
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.min.y,
                self.box_.min.z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.min.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.min.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.min.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.min.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.max.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.max.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.max.y,
                self.box_.min.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y - corner_expansion_y,
                self.box_.max.z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.min.x, self.box_.max.y, self.box_.max.z),
            vec3(
                self.box_.min.x,
                self.box_.max.y,
                self.box_.max.z - corner_expansion_z,
            ),
            vec3(
                self.box_.min.x + corner_expansion_x,
                self.box_.max.y,
                self.box_.max.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y + corner_expansion_y,
                self.box_.min.z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.min.y,
                self.box_.min.z,
            ),
            vec3(self.box_.max.x, self.box_.min.y, self.box_.min.z),
            vec3(
                self.box_.max.x,
                self.box_.min.y,
                self.box_.min.z + corner_expansion_z,
            ),
            vec3(
                self.box_.max.x - corner_expansion_x,
                self.box_.min.y,
                self.box_.min.z,
            ),
        ]
    }
}
