pub struct Transform {
    pub position: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

pub struct Object {
    transform: Transform,
    center: glam::Vec3,
}

impl Object {
    pub fn rotate(&mut self, rotation: glam::Quat) {
        self.transform.rotation = rotation * self.transform.rotation;
    }

    pub fn translate(&mut self, translation: glam::Vec3) {
        self.transform.position += translation;
    }
}
