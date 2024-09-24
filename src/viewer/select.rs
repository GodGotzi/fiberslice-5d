use std::sync::Arc;

use rether::{alloc::DynamicAllocHandle, model::Model, vertex::Vertex, Transform};

#[derive(Debug)]
pub enum TransformResponse {
    None,
    Translate,
    Rotate,
    Scale,
}

#[derive(Default)]
pub struct Selector {
    selected: Vec<Arc<dyn Model<Vertex, DynamicAllocHandle<Vertex>>>>,
}

impl std::fmt::Debug for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Selector")
            .field("selected", &self.selected.len())
            .finish()
    }
}

impl Selector {
    pub fn select(&mut self, model: &Arc<dyn Model<Vertex, DynamicAllocHandle<Vertex>>>) {
        self.selected.push(model.clone());
    }

    pub fn deselect(&mut self, model: &Arc<dyn Model<Vertex, DynamicAllocHandle<Vertex>>>) {
        self.selected.retain(|m| !Arc::ptr_eq(m, model));
    }

    pub fn transform(&self, mut r#fn: impl FnMut(&mut Transform) -> TransformResponse) {
        if self.selected.len() == 1 {
            let mut transform = self.selected[0].transform();

            let before = transform.clone();

            let response = r#fn(&mut transform);

            let translation = transform.translation - before.translation;
            let rotation = transform.rotation * before.rotation;
            let scale = transform.scale / before.scale;

            match response {
                TransformResponse::None => (),
                TransformResponse::Translate => {
                    if translation != glam::Vec3::ZERO {
                        self.selected[0].translate(translation);
                    }
                }
                TransformResponse::Rotate => {
                    if rotation != glam::Quat::IDENTITY {
                        self.selected[0].rotate(rotation, None);
                    }
                }
                TransformResponse::Scale => {
                    if scale != glam::Vec3::ONE {
                        self.selected[0].scale(scale, None);
                    }
                }
            }
        } else {
            let mut transform = Transform::default();

            let response = r#fn(&mut transform);

            for model in &self.selected {
                match response {
                    TransformResponse::None => (),
                    TransformResponse::Translate => {
                        if transform.translation != glam::Vec3::ZERO {
                            model.translate(transform.translation);
                        }
                    }
                    TransformResponse::Rotate => {
                        if transform.rotation != glam::Quat::IDENTITY {
                            model.rotate(transform.rotation, None);
                        }
                    }
                    TransformResponse::Scale => {
                        if transform.scale != glam::Vec3::ONE {
                            model.scale(transform.scale, None);
                        }
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn selected(&self) -> &[Arc<dyn Model<Vertex, DynamicAllocHandle<Vertex>>>] {
        &self.selected
    }
}
