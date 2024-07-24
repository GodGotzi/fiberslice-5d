use glam::{Quat, Vec3};
use log::info;

use crate::{
    model::{
        transform::{Rotate, Scale, Translate},
        Expandable,
    },
    prelude::WgpuContext,
    GlobalState, RootEvent,
};

use super::{
    hitbox::{Hitbox, PickContext},
    ray::Ray,
};

pub trait Pickable: Hitbox {
    fn picked(&self, global_state: &GlobalState<RootEvent>, wgpu_context: &WgpuContext);
}

impl Translate for PickContext {
    fn translate(&mut self, translation: Vec3) {
        self.write().translate(translation)
    }
}

impl Rotate for PickContext {
    fn rotate(&mut self, rotation: Quat) {
        self.write().rotate(rotation)
    }
}

impl Scale for PickContext {
    fn scale(&mut self, scale: Vec3) {
        self.write().scale(scale)
    }
}

impl Hitbox for PickContext {
    fn check_hit(&self, ray: &Ray, wgpu_context: &WgpuContext) -> Option<f32> {
        self.read().check_hit(ray, wgpu_context)
    }

    fn expand(&mut self, _box: &dyn Hitbox) {
        self.write().expand(_box)
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.write().set_enabled(enabled)
    }

    fn enabled(&self) -> bool {
        self.read().enabled()
    }

    fn min(&self) -> Vec3 {
        self.read().min()
    }

    fn max(&self) -> Vec3 {
        self.read().max()
    }
}

impl Pickable for PickContext {
    fn picked(&self, global_state: &GlobalState<RootEvent>, wgpu_context: &WgpuContext) {
        self.read().picked(global_state, wgpu_context)
    }
}

impl Expandable for PickContext {
    fn expand(&mut self, _box: &Self) {
        self.write().expand(_box)
    }
}
