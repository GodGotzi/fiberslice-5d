use bevy::prelude::Vec3;

use crate::api::Flip;

pub struct FSVec3(pub Vec3);

impl From<FSVec3> for [f32; 3] {
    fn from(value: FSVec3) -> Self {
        [value.0.x, value.0.z, value.0.y]
    }
}

impl Flip for (Vec3, Vec3, Vec3) {
    fn flip(&mut self) {
        std::mem::swap(&mut self.0, &mut self.2);
    }
}
