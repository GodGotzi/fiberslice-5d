use glam::{Quat, Vec2, Vec3};
use rether::{
    picking::{Hitbox, Ray},
    transform::{Rotate, Scale, Translate},
    Expandable, TreeHandle,
};
use winit::event::MouseButton;

use crate::prelude::SharedMut;

pub trait Interactive: Hitbox {
    fn mouse_clicked(&mut self, button: MouseButton) {}
    fn mouse_scroll(&mut self, delta: f32) {}
    fn mouse_motion(&mut self, button: MouseButton, delta: Vec2) {}
}

impl<C: Interactive + Hitbox> Interactive for TreeHandle<C> {
    fn mouse_clicked(&mut self, button: winit::event::MouseButton) {
        match self {
            Self::Root { ctx, .. } => ctx.mouse_clicked(button),
            Self::Node { ctx, .. } => ctx.mouse_clicked(button),
        }
    }

    fn mouse_motion(&mut self, button: winit::event::MouseButton, delta: glam::Vec2) {
        match self {
            Self::Root { ctx, .. } => ctx.mouse_motion(button, delta),
            Self::Node { ctx, .. } => ctx.mouse_motion(button, delta),
        }
    }

    fn mouse_scroll(&mut self, delta: f32) {
        match self {
            Self::Root { ctx, .. } => ctx.mouse_scroll(delta),
            Self::Node { ctx, .. } => ctx.mouse_scroll(delta),
        }
    }
}

pub type InteractContext = SharedMut<Box<dyn Interactive>>;

impl Translate for InteractContext {
    fn translate(&mut self, translation: Vec3) {
        self.write().translate(translation)
    }
}

impl Rotate for InteractContext {
    fn rotate(&mut self, rotation: Quat) {
        self.write().rotate(rotation)
    }
}

impl Scale for InteractContext {
    fn scale(&mut self, scale: Vec3) {
        self.write().scale(scale)
    }
}

impl Hitbox for InteractContext {
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

impl Expandable for InteractContext {
    fn expand(&mut self, _box: &Self) {
        self.write().expand(_box)
    }
}
