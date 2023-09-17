use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use three_d::*;

use crate::{
    application::{Application, AsyncManipulator},
    model::layer::ToolPathModel,
};

use super::{environment, Mode};
use three_d_asset::TriMesh;

pub type ModelMap = Arc<Mutex<HashMap<String, HideableObject<TriMesh>>>>;
pub type ObjectMap = Arc<Mutex<HashMap<String, HideableObject<dyn Object>>>>;

type ModelManipulator = AsyncManipulator<ModelMap>;
type ObjectManipulator = AsyncManipulator<ObjectMap>;

pub struct BufferManipulator {
    pub model_manipulator: ModelManipulator,
    pub object_manipulator: ObjectManipulator,
}

impl BufferManipulator {
    pub fn new() -> Self {
        Self {
            model_manipulator: AsyncManipulator::new(Vec::new()),
            object_manipulator: AsyncManipulator::new(Vec::new()),
        }
    }

    pub fn update_models(&mut self, models: Arc<Mutex<HashMap<String, HideableObject<TriMesh>>>>) {
        self.model_manipulator.next_frame(models);
    }

    pub fn update_objects(
        &mut self,
        objects: Arc<Mutex<HashMap<String, HideableObject<dyn Object>>>>,
    ) {
        self.object_manipulator.next_frame(objects);
    }
}

pub struct HideableObject<O: ?Sized + 'static> {
    inner: Box<O>,
    visible: bool,
}

#[allow(dead_code)]
impl<O: ?Sized + 'static> HideableObject<O> {
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

pub struct ObjectBuffer<'a, O: Object + ?Sized + 'static> {
    skybox: Option<Skybox>,
    toolpath_model: Option<ToolPathModel<'a>>,
    models: Arc<Mutex<HashMap<String, HideableObject<TriMesh>>>>,
    objects: Arc<Mutex<HashMap<String, HideableObject<O>>>>,
    interactive_objects: HashMap<String, HideableObject<O>>,
}

impl<O: Object + ?Sized + 'static> Default for ObjectBuffer<'_, O> {
    fn default() -> Self {
        Self {
            skybox: None,
            toolpath_model: None,
            models: Arc::new(Mutex::new(HashMap::new())),
            objects: Arc::new(Mutex::new(HashMap::new())),
            interactive_objects: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl<'a, O: Object + ?Sized + 'static> ObjectBuffer<'a, O> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn models(&self) -> Arc<Mutex<HashMap<String, HideableObject<TriMesh>>>> {
        self.models.clone()
    }

    pub fn objects(&self) -> Arc<Mutex<HashMap<String, HideableObject<O>>>> {
        self.objects.clone()
    }

    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = Some(skybox);
    }

    pub fn set_toolpath_model(&mut self, toolpath_model: ToolPathModel<'a>) {
        self.toolpath_model = Some(toolpath_model);
    }

    pub fn add_model<S: Into<String>>(&mut self, name: S, model: Box<TriMesh>) {
        self.models
            .lock()
            .unwrap()
            .insert(name.into(), HideableObject::new(model));
    }

    pub fn add_model_and_hide<S: Into<String>>(&mut self, name: S, model: Box<TriMesh>) {
        let mut object = HideableObject::new(model);
        object.hide();

        self.models.lock().unwrap().insert(name.into(), object);
    }

    pub fn add_object<S: Into<String>>(&mut self, name: S, object: Box<O>) {
        self.objects
            .lock()
            .unwrap()
            .insert(name.into(), HideableObject::new(object));
    }

    pub fn add_object_and_hide<S: Into<String>>(&mut self, name: S, object: Box<O>) {
        let mut object = HideableObject::new(object);
        object.hide();

        self.objects.lock().unwrap().insert(name.into(), object);
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

    pub fn get_interactive_object<S: Into<&'a String>>(&self, name: S) -> Option<&O> {
        if let Some(object) = self.interactive_objects.get(name.into()) {
            Some(&object.inner)
        } else {
            None
        }
    }

    pub fn remove_model<S: Into<&'a String>>(&mut self, name: S) -> Box<TriMesh> {
        self.models
            .lock()
            .unwrap()
            .remove(name.into())
            .unwrap()
            .inner
    }

    pub fn remove_object<S: Into<&'a String>>(&mut self, name: S) -> Box<O> {
        self.objects
            .lock()
            .unwrap()
            .remove(name.into())
            .unwrap()
            .inner
    }

    pub fn remove_interactive_object<S: Into<&'a String>>(&mut self, name: S) -> Box<O> {
        self.interactive_objects.remove(name.into()).unwrap().inner
    }

    pub fn hide_model<S: Into<&'a String>>(&mut self, name: S) {
        if let Some(model) = self.models.lock().unwrap().get_mut(name.into()) {
            model.hide();
        }
    }

    pub fn hide_object<S: Into<&'a String>>(&mut self, name: S) {
        if let Some(object) = self.objects.lock().unwrap().get_mut(name.into()) {
            object.hide();
        }
    }

    pub fn hide_interactive_object<S: Into<&'a String>>(&mut self, name: S) {
        if let Some(object) = self.interactive_objects.get_mut(name.into()) {
            object.hide();
        }
    }

    pub fn clear_models(&mut self) {
        self.models.lock().unwrap().clear();
    }

    pub fn clear_objects(&mut self) {
        self.objects.lock().unwrap().clear();
    }

    pub fn clear_interactive_objects(&mut self) {
        self.interactive_objects.clear();
    }

    pub fn clear(&mut self) {
        self.clear_models();
        self.clear_objects();
        self.clear_interactive_objects();
    }

    pub fn render(
        &self,
        environment: &environment::Environment,
        application: &Application,
        context: Context,
    ) {
        if let Some(ref skybox) = self.skybox {
            skybox.render(environment.camera(), &[]);
        }

        if application.context().is_mode(Mode::Preview) {
            if let Some(toolpath) = self.toolpath_model.as_ref() {
                toolpath
                    .model
                    .render(environment.camera(), environment.lights().as_slice());
            }
        }

        for model in self.models.lock().unwrap().values() {
            if model.is_visible() {
                let trimesh = model.object();

                let model = Gm::new(
                    Mesh::new(&context, trimesh),
                    PhysicalMaterial {
                        albedo: Srgba::WHITE,
                        ..Default::default()
                    },
                );

                model.render(environment.camera(), environment.lights().as_slice());
            }
        }

        for object in self.objects.lock().unwrap().values() {
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

    pub fn check_picks(
        &mut self,
        context: &WindowedContext,
        frame_input: &FrameInput,
        environment: &environment::Environment,
    ) {
        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button, position, ..
            } = event
            {
                if *button == MouseButton::Right {
                    if let Some(s) = pick(
                        context,
                        environment.camera(),
                        position,
                        self.toolpath_model
                            .as_ref()
                            .unwrap()
                            .model
                            .geometry
                            .into_iter(),
                    ) {
                        println!("Pick: {:?}", s);
                    }
                }
            }
        }
    }
}
