use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualPlane {
    position: Vec3,
    normal: Vec3,
}

#[allow(dead_code)]
impl VirtualPlane {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}
