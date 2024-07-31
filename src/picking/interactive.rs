use glam::{Quat, Vec2, Vec3};
use winit::event::MouseButton;

use crate::{
    model::{
        transform::{Rotate, Scale, Translate},
        Expandable,
    },
    prelude::WgpuContext,
};

use super::{
    hitbox::{Hitbox, InteractiveContext},
    ray::Ray,
};

pub trait Interactive: Hitbox {
    fn mouse_clicked(
        &mut self,
        button: MouseButton,
        global_state: crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
    }
    fn mouse_scroll(
        &mut self,
        delta: f32,
        global_state: crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
    }
    fn mouse_delta(
        &mut self,
        button: MouseButton,
        delta: Vec2,
        global_state: crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
    }
}

impl Translate for InteractiveContext {
    fn translate(&mut self, translation: Vec3) {
        self.write().translate(translation)
    }
}

impl Rotate for InteractiveContext {
    fn rotate(&mut self, rotation: Quat) {
        self.write().rotate(rotation)
    }
}

impl Scale for InteractiveContext {
    fn scale(&mut self, scale: Vec3) {
        self.write().scale(scale)
    }
}

impl Hitbox for InteractiveContext {
    fn check_hit(&self, ray: &Ray) -> Option<f32> {
        self.read().check_hit(ray)
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

impl Interactive for InteractiveContext {
    fn mouse_clicked(
        &mut self,
        button: MouseButton,
        global_state: crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
        self.write()
            .mouse_clicked(button, global_state, wgpu_context)
    }

    fn mouse_scroll(
        &mut self,
        delta: f32,
        global_state: crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
        self.write().mouse_scroll(delta, global_state, wgpu_context)
    }

    fn mouse_delta(
        &mut self,
        button: MouseButton,
        delta: Vec2,
        global_state: crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
        self.write()
            .mouse_delta(button, delta, global_state, wgpu_context)
    }
}

impl Expandable for InteractiveContext {
    fn expand(&mut self, _box: &Self) {
        self.write().expand(_box)
    }
}
