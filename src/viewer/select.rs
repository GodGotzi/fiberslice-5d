use glam::Mat4;
use rether::{vertex::Vertex, Transform};

use crate::{
    model::Model,
    prelude::{ArcModel, SharedMut},
};

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
    selected: Vec<ArcModel<CADModel>>,
    grouped_transform: Option<Mat4>,
}

impl std::fmt::Debug for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Selector")
            .field("selected", &self.selected.len())
            .finish()
    }
}

impl Selector {
    pub fn select(&mut self, model: &ArcModel<CADModel>) {
        self.selected.push(model.clone());

        self.grouped_transform = None;
    }

    pub fn deselect(&mut self, model: &ArcModel<CADModel>) {
        let size = self.selected.len();
        self.selected.retain(|m| !ArcModel::ptr_eq(m, model));

        if size != self.selected.len() {
            self.grouped_transform = None;
        }
    }

    pub fn transform(&mut self, mut r#fn: impl FnMut(&mut Mat4) -> bool) {
        if self.selected.len() == 1 {
            let mut transform = self.selected[0].read().get_transform();

            let response = r#fn(&mut transform);

            if response {
                self.selected[0].write().transform(transform);
            }
        } else {
            let mut transform = self
                .grouped_transform
                .unwrap_or(Mat4::from_translation(glam::Vec3::ZERO));

            let response = r#fn(&mut transform);

            if response {
                for model in &self.selected {
                    let (scale, rotate, translate) =
                        model.read().get_transform().to_scale_rotation_translation();

                    let (grouped_scale, grouped_rotate, grouped_translate) = (transform.inverse()
                        * self
                            .grouped_transform
                            .unwrap_or(Mat4::from_translation(glam::Vec3::ZERO)))
                    .to_scale_rotation_translation();

                    let transform = Mat4::from_scale_rotation_translation(
                        scale * grouped_scale,
                        rotate * grouped_rotate,
                        translate + grouped_translate,
                    );

                    model.write().transform(transform);
                }
            }

            self.grouped_transform = Some(transform);
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn selected(&self) -> &[ArcModel<CADModel>] {
        &self.selected
    }
}
