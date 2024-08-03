use glam::{vec4, Vec3};
use rether::{
    picking::{Hitbox, Ray},
    transform::{Rotate, Scale, Translate},
    vertex::Vertex,
};

use crate::viewer::{ToVisual, Visual};

use super::{
    mesh::{Mesh, WireMesh},
    QuadFace, SelectBox,
};

#[derive(Debug, Clone, Copy)]
pub struct BoundingHitbox {
    pub max: Vec3,
    pub min: Vec3,
    enabled: bool,
}

impl Default for BoundingHitbox {
    fn default() -> Self {
        Self {
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            enabled: true,
        }
    }
}

impl BoundingHitbox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            max,
            min,
            enabled: true,
        }
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

    pub fn expand_min(&mut self, min: Vec3) {
        self.min = self.min.min(min);
    }

    pub fn expand_max(&mut self, max: Vec3) {
        self.max = self.max.max(max);
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
                point: Vec3::new(self.max.x, self.max.y, self.max.z),
                max: Vec3::new(self.max.x, self.max.y, self.max.z),
                min: Vec3::new(self.max.x, self.min.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(-1.0, 0.0, 0.0),
                point: Vec3::new(self.min.x, self.max.y, self.max.z),
                max: Vec3::new(self.min.x, self.max.y, self.max.z),
                min: Vec3::new(self.min.x, self.min.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, 1.0, 0.0),
                point: Vec3::new(self.max.x, self.max.y, self.max.z),
                max: Vec3::new(self.max.x, self.max.y, self.max.z),
                min: Vec3::new(self.min.x, self.max.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, -1.0, 0.0),
                point: Vec3::new(self.max.x, self.min.y, self.max.z),
                max: Vec3::new(self.max.x, self.min.y, self.max.z),
                min: Vec3::new(self.min.x, self.min.y, self.min.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, 0.0, 1.0),
                point: Vec3::new(self.max.x, self.max.y, self.max.z),
                max: Vec3::new(self.max.x, self.max.y, self.max.z),
                min: Vec3::new(self.min.x, self.min.y, self.max.z),
            },
            QuadFace {
                normal: Vec3::new(0.0, 0.0, -1.0),
                point: Vec3::new(self.max.x, self.max.y, self.min.z),
                max: Vec3::new(self.max.x, self.max.y, self.min.z),
                min: Vec3::new(self.min.x, self.min.y, self.min.z),
            },
        ]
    }
}

impl Translate for BoundingHitbox {
    fn translate(&mut self, translation: Vec3) {
        self.min += translation;
        self.max += translation;
    }
}

impl Rotate for BoundingHitbox {
    fn rotate(&mut self, _rotation: glam::Quat) {
        todo!("Implement rotate for BoundingHitbox")
    }
}

impl Scale for BoundingHitbox {
    fn scale(&mut self, _scale: Vec3) {
        todo!("Implement scale for BoundingHitbox")
    }
}

impl Hitbox for BoundingHitbox {
    fn check_hit(&self, ray: &Ray) -> Option<f32> {
        // bounding box min max

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

        if min.is_some() {
            println!("Ray intersects bounding box");

            println!("Distance: {:?}", min.unwrap());
        }

        min
    }

    fn expand(&mut self, box_: &dyn Hitbox) {
        self.min = self.min.min(box_.min());
        self.max = self.max.max(box_.max());
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn min(&self) -> Vec3 {
        self.min
    }

    fn max(&self) -> Vec3 {
        self.max
    }
}

impl ToVisual<72, 48> for BoundingHitbox {
    fn to_visual(&self) -> Visual<72, 48> {
        let diagonal = self.max() - self.min();
        let distance = diagonal.x.min(diagonal.y).min(diagonal.z);

        let select_smaller_box: SelectBox = SelectBox::from(BoundingHitbox::new(
            self.min() - distance * 0.1,
            self.max() + distance * 0.1,
        ))
        .with_color(vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 1.0, 1.0));

        let mut wires = [Vertex::default(); 48];

        wires[..25].copy_from_slice(&select_smaller_box.to_wire_vertices());

        Visual {
            vertices: select_smaller_box.to_triangle_vertices(),
            wires,
        }
    }
}
