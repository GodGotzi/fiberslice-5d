use std::collections::HashMap;

use three_d::*;

use super::environment;

pub struct HideableObject<O: Object + 'static> {
    inner: O,
    visible: bool,
}

#[allow(dead_code)]
impl<O: Object + 'static> HideableObject<O> {
    pub fn new(object: O) -> Self {
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

pub struct ObjectBuffer<O: Object + 'static> {
    layers: Vec<O>,
    models: HashMap<String, HideableObject<O>>,
    objects: HashMap<String, HideableObject<O>>,
    interactive_objects: HashMap<String, HideableObject<O>>,
}

#[allow(dead_code)]
impl<'a, O: Object + 'static> ObjectBuffer<O> {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            models: HashMap::new(),
            objects: HashMap::new(),
            interactive_objects: HashMap::new(),
        }
    }

    pub fn add_layer(&mut self, layer: O) {
        self.layers.push(layer);
    }

    pub fn add_model<S: Into<String>>(&mut self, name: S, model: O) {
        self.models.insert(name.into(), HideableObject::new(model));
    }

    pub fn add_model_and_hide<S: Into<String>>(&mut self, name: S, model: O) {
        let mut object = HideableObject::new(model);
        object.hide();

        self.models.insert(name.into(), object);
    }

    pub fn add_object<S: Into<String>>(&mut self, name: S, object: O) {
        self.objects
            .insert(name.into(), HideableObject::new(object));
    }

    pub fn add_object_and_hide<S: Into<String>>(&mut self, name: S, object: O) {
        let mut object = HideableObject::new(object);
        object.hide();

        self.objects.insert(name.into(), object);
    }

    pub fn add_interactive_object<S: Into<String>>(&mut self, name: S, object: O) {
        self.interactive_objects
            .insert(name.into(), HideableObject::new(object));
    }

    pub fn add_interactive_object_and_hide<S: Into<String>>(&mut self, name: S, object: O) {
        let mut object = HideableObject::new(object);
        object.hide();

        self.interactive_objects.insert(name.into(), object);
    }

    pub fn get_layer(&self, index: usize) -> Option<&O> {
        self.layers.get(index)
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

    pub fn remove_layer(&mut self, index: usize) -> O {
        self.layers.remove(index)
    }

    pub fn remove_model<S: Into<&'a String>>(&mut self, name: S) -> O {
        self.models.remove(name.into()).unwrap().inner
    }

    pub fn remove_object<S: Into<&'a String>>(&mut self, name: S) -> O {
        self.objects.remove(name.into()).unwrap().inner
    }

    pub fn remove_interactive_object<S: Into<&'a String>>(&mut self, name: S) -> O {
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

    pub fn render(&self, environment: &environment::Environment) {
        for layer in &self.layers {
            layer.render(environment.camera(), environment.lights().as_slice());
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
