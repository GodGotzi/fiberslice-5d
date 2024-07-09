use glam::Vec3;

use crate::{picking::hitbox::Hitbox, prelude::SharedMut};

use super::QuadFace;

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
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    pub fn expand_point(&mut self, point: Vec3) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    pub fn contains(&self, point: Vec3) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.y <= point.y
            && point.y <= self.max.y
            && self.min.z <= point.z
            && point.z <= self.max.z
    }

    pub fn faces(&self) -> [QuadFace; 6] {
        [
            QuadFace {
                normal: Vec3::new(1.0, 0.0, 0.0),
                max: Vec3::new(self.max.x, self.max.y, self.max.z),
                min: Vec3::new(self.max.x, self.min.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(-1.0, 0.0, 0.0),
                max: Vec3::new(self.min.x, self.max.y, self.max.z),
                min: Vec3::new(self.min.x, self.min.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, 1.0, 0.0),
                max: Vec3::new(self.max.x, self.max.y, self.max.z),
                min: Vec3::new(self.min.x, self.max.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, -1.0, 0.0),
                max: Vec3::new(self.max.x, self.min.y, self.max.z),
                min: Vec3::new(self.min.x, self.min.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, 0.0, 1.0),
                max: Vec3::new(self.max.x, self.max.y, self.max.z),
                min: Vec3::new(self.min.x, self.min.y, self.max.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, 0.0, -1.0),
                max: Vec3::new(self.max.x, self.max.y, self.min.z),
                min: Vec3::new(self.min.x, self.min.y, self.min.z),
            },
        ]
    }
}

impl Hitbox for BoundingBox {
    fn check_hit(&self, ray: &crate::picking::ray::Ray) -> Option<f32> {
        if self.contains(ray.origin) {
            return Some(0.0);
        }

        let mut min = None;

        for quad_face in self.faces() {
            let distance = quad_face.check_hit(ray);

            if let Some(distance) = distance {
                if min.unwrap_or(f32::MAX) > distance || min.is_none() {
                    min = Some(distance);
                }
            }
        }

        min
    }

    fn expand(&mut self, _box: &SharedMut<Box<dyn Hitbox>>) {
        self.min = self.min.min(_box.read().min());
        self.max = self.max.max(_box.read().max());
    }

    fn min(&self) -> Vec3 {
        self.min
    }

    fn max(&self) -> Vec3 {
        self.max
    }
}
