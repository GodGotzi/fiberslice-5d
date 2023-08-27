use std::collections::HashMap;

use three_d::*;

use crate::application::Application;

use super::{environment, Mode};

pub struct HideableObject<O: Object + ?Sized + 'static> {
    inner: Box<O>,
    visible: bool,
}

#[allow(dead_code)]
impl<O: Object + ?Sized + 'static> HideableObject<O> {
    pub fn new(object: Box<O>) -> Self {
        Self {
            inner: object,
            visible: true,
        }
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn object(&self) -> &O {
        &self.inner
    }
}

pub struct ObjectBuffer<O: Object + ?Sized + 'static> {
    skybox: Option<Skybox>,
    layers: Vec<HideableObject<O>>,
    models: HashMap<String, HideableObject<O>>,
    objects: HashMap<String, HideableObject<O>>,
    interactive_objects: HashMap<String, HideableObject<O>>,
}

impl<O: Object + ?Sized + 'static> Default for ObjectBuffer<O> {
    fn default() -> Self {
        Self {
            skybox: None,
            layers: Vec::new(),
            models: HashMap::new(),
            objects: HashMap::new(),
            interactive_objects: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl<'a, O: Object + ?Sized + 'static> ObjectBuffer<O> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = Some(skybox);
    }

    pub fn add_layer(&mut self, layer: Box<O>) {
        self.layers.push(HideableObject::new(layer));
    }

    pub fn add_model<S: Into<String>>(&mut self, name: S, model: Box<O>) {
        self.models.insert(name.into(), HideableObject::new(model));
    }

    pub fn add_model_and_hide<S: Into<String>>(&mut self, name: S, model: Box<O>) {
        let mut object = HideableObject::new(model);
        object.hide();

        self.models.insert(name.into(), object);
    }

    pub fn add_object<S: Into<String>>(&mut self, name: S, object: Box<O>) {
        self.objects
            .insert(name.into(), HideableObject::new(object));
    }

    pub fn add_object_and_hide<S: Into<String>>(&mut self, name: S, object: Box<O>) {
        let mut object = HideableObject::new(object);
        object.hide();

        self.objects.insert(name.into(), object);
    }

    pub fn add_interactive_object<S: Into<String>>(&mut self, name: S, object: Box<O>) {
        self.interactive_objects
            .insert(name.into(), HideableObject::new(object));
    }

    pub fn add_interactive_object_and_hide<S: Into<String>>(&mut self, name: S, object: Box<O>) {
        let mut object = HideableObject::new(object);
        object.hide();

        self.interactive_objects.insert(name.into(), object);
    }

    pub fn get_layer(&self, index: usize) -> Option<&O> {
        if let Some(layer) = self.layers.get(index) {
            Some(&layer.inner)
        } else {
            None
        }
    }

    pub fn get_model<S: Into<&'a String>>(&self, name: S) -> Option<&O> {
        if let Some(model) = self.models.get(name.into()) {
            Some(&model.inner)
        } else {
            None
        }
    }

    pub fn get_object<S: Into<&'a String>>(&self, name: S) -> Option<&O> {
        if let Some(object) = self.objects.get(name.into()) {
            Some(&object.inner)
        } else {
            None
        }
    }

    pub fn get_interactive_object<S: Into<&'a String>>(&self, name: S) -> Option<&O> {
        if let Some(object) = self.interactive_objects.get(name.into()) {
            Some(&object.inner)
        } else {
            None
        }
    }

    pub fn remove_layer(&mut self, index: usize) -> Box<O> {
        self.layers.remove(index).inner
    }

    pub fn remove_model<S: Into<&'a String>>(&mut self, name: S) -> Box<O> {
        self.models.remove(name.into()).unwrap().inner
    }

    pub fn remove_object<S: Into<&'a String>>(&mut self, name: S) -> Box<O> {
        self.objects.remove(name.into()).unwrap().inner
    }

    pub fn remove_interactive_object<S: Into<&'a String>>(&mut self, name: S) -> Box<O> {
        self.interactive_objects.remove(name.into()).unwrap().inner
    }

    pub fn hide_model<S: Into<&'a String>>(&mut self, name: S) {
        if let Some(model) = self.models.get_mut(name.into()) {
            model.hide();
        }
    }

    pub fn hide_object<S: Into<&'a String>>(&mut self, name: S) {
        if let Some(object) = self.objects.get_mut(name.into()) {
            object.hide();
        }
    }

    pub fn hide_interactive_object<S: Into<&'a String>>(&mut self, name: S) {
        if let Some(object) = self.interactive_objects.get_mut(name.into()) {
            object.hide();
        }
    }

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    pub fn clear_models(&mut self) {
        self.models.clear();
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn clear_interactive_objects(&mut self) {
        self.interactive_objects.clear();
    }

    pub fn clear(&mut self) {
        self.clear_layers();
        self.clear_models();
        self.clear_objects();
        self.clear_interactive_objects();
    }

    pub fn render(&self, environment: &environment::Environment, application: &Application) {
        if let Some(ref skybox) = self.skybox {
            skybox.render(environment.camera(), &[]);
        }

        if application.context().is_mode(Mode::Preview) {
            for layer in &self.layers {
                layer
                    .inner
                    .render(environment.camera(), environment.lights().as_slice());
            }
        }

        for model in self.models.values() {
            if model.is_visible() {
                model
                    .object()
                    .render(environment.camera(), environment.lights().as_slice());
            }
        }

        for object in self.objects.values() {
            if object.is_visible() {
                object
                    .object()
                    .render(environment.camera(), environment.lights().as_slice());
            }
        }

        for object in self.interactive_objects.values() {
            if object.is_visible() {
                object
                    .object()
                    .render(environment.camera(), environment.lights().as_slice());
            }
        }
    }
}
