use serde::{Deserialize, Serialize};
use three_d_asset::Vector3;

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualPlane {
    position: Vector3<f32>,
    normal: Vector3<f32>,
}

impl VirtualPlane {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self { position, normal }
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn normal(&self) -> &Vector3<f32> {
        &self.normal
    }
}
