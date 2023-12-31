use three_d::Vector3;

pub type Triangle3d = (Vector3<f32>, Vector3<f32>, Vector3<f32>);

pub struct Rect3d {
    pub left_0: Vector3<f32>,
    pub left_1: Vector3<f32>,
    pub right_0: Vector3<f32>,
    pub right_1: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct VirtualBox {
    max: Vector3<f32>,
    min: Vector3<f32>,
}

impl Default for VirtualBox {
    fn default() -> Self {
        Self {
            max: Vector3::new(f32::MIN, f32::MIN, f32::MIN),
            min: Vector3::new(f32::MAX, f32::MAX, f32::MAX),
        }
    }
}

impl VirtualBox {
    pub fn new(max: Vector3<f32>, min: Vector3<f32>) -> Self {
        Self { max, min }
    }

    pub fn expand(&mut self, other: Self) {
        self.max.x = self.max.x.max(other.max.x);
        self.max.y = self.max.y.max(other.max.y);
        self.max.z = self.max.z.max(other.max.z);

        self.min.x = self.min.x.min(other.min.x);
        self.min.y = self.min.y.min(other.min.y);
        self.min.z = self.min.z.min(other.min.z);
    }
}
