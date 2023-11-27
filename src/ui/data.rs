use std::sync::{Arc, Mutex, MutexGuard};

use crate::view::Mode;

use super::{boundary::Boundary, Theme};

pub struct UiData {
    pub theme: Theme,
    pub mode: Mode,

    pub context: UiContext,
}

impl Default for UiData {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
        }
    }
}

#[derive(Clone)]
pub struct UiContext {
    component_data: Arc<Mutex<ComponentDataHolder>>,
}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            component_data: Arc::new(Mutex::new(ComponentDataHolder::default())),
        }
    }
}

impl UiContext {
    pub fn get_component_data_mut(&self) -> MutexGuard<ComponentDataHolder> {
        self.component_data.lock().unwrap()
    }
}

pub struct ComponentData {
    pub boundary: Option<Boundary>,
    pub enabled: bool,
}

impl ComponentData {
    fn delete_cache(&mut self) {
        self.boundary = None;
    }

    pub fn boundary(&self) -> Boundary {
        self.boundary.unwrap_or(Boundary::zero())
    }

    pub fn set_boundary(&mut self, boundary: Boundary) {
        self.boundary = Some(boundary);
    }
}

impl Default for ComponentData {
    fn default() -> Self {
        Self {
            boundary: None,
            enabled: true,
        }
    }
}

#[derive(Default)]
pub struct ComponentDataHolder {
    pub menubar: ComponentData,
    pub taskbar: ComponentData,
    pub modebar: ComponentData,
    pub toolbar: ComponentData,
    pub settingsbar: ComponentData,
    pub addons: ComponentData,
}

impl ComponentDataHolder {
    pub fn delete_cache(&mut self) {
        self.menubar.delete_cache();
        self.taskbar.delete_cache();
        self.modebar.delete_cache();
        self.toolbar.delete_cache();
        self.settingsbar.delete_cache();
    }
}
