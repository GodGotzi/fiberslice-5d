use glam::Mat4;
use rether::{vertex::Vertex, Transform};

use crate::{model::Model, prelude::SharedMut};

use super::server::model::CADModel;

#[derive(Debug)]
pub enum TransformResponse {
    None,
    Translate,
    Rotate,
    Scale,
}

#[derive(Default)]
pub struct Selector {
    selected: Vec<SharedMut<CADModel>>,
}

impl std::fmt::Debug for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Selector")
            .field("selected", &self.selected.len())
            .finish()
    }
}

impl Selector {
    pub fn select(&mut self, model: &SharedMut<CADModel>) {
        self.selected.push(model.clone());
    }

    pub fn deselect(&mut self, model: &SharedMut<CADModel>) {
        self.selected.retain(|m| !SharedMut::ptr_eq(m, model));
    }

    pub fn transform(&self, mut r#fn: impl FnMut(&mut Mat4) -> bool) {
        if self.selected.len() == 1 {
            let mut transform = self.selected[0].read().get_transform();

            let response = r#fn(&mut transform);

            if response {
                self.selected[0].write().transform(transform);
            }
        } else {
            let mut transform = Mat4::from_translation(glam::Vec3::ZERO);

            let response = r#fn(&mut transform);

            if response {
                for model in &self.selected {
                    model.write().transform(transform);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn selected(&self) -> &[SharedMut<CADModel>] {
        &self.selected
    }
}
