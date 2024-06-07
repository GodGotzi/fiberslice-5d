use glam::Vec3;

pub type Triangle3d = (Vec3, Vec3, Vec3);

pub struct Rect3d {
    pub left_0: Vec3,
    pub left_1: Vec3,
    pub right_0: Vec3,
    pub right_1: Vec3,
}

#[derive(Debug, Clone)]
pub struct VirtualBox {
    max: Vec3,
    min: Vec3,
}

impl Default for VirtualBox {
    fn default() -> Self {
        Self {
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
        }
    }
}

impl VirtualBox {
    pub fn new(max: Vec3, min: Vec3) -> Self {
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
