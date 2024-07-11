use glam::Mat4;

#[derive(Debug, Clone, Default)]
pub struct Transform {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

pub trait Translate {
    fn translate(&mut self, translation: glam::Vec3);
}

pub trait Rotate {
    fn rotate(&mut self, rotation: glam::Quat);
}

pub trait Scale {
    fn scale(&mut self, scale: glam::Vec3);
}

impl Translate for Transform {
    fn translate(&mut self, translation: glam::Vec3) {
        self.translation += translation;
    }
}

impl Rotate for Transform {
    fn rotate(&mut self, rotation: glam::Quat) {
        self.rotation = rotation * self.rotation;
    }
}

impl Scale for Transform {
    fn scale(&mut self, scale: glam::Vec3) {
        self.scale *= scale;
    }
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_translation(self.translation)
            * Mat4::from_quat(self.rotation)
            * Mat4::from_scale(self.scale)
    }
}
