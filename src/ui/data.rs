use std::sync::{Arc, Mutex, MutexGuard};

use crate::{prelude::ApplicationState, view::Mode};

use super::{boundary::Boundary, response::Responses, Theme};

pub struct UiData {
    pub theme: Theme,
    pub mode: Mode,

    pub responses: Arc<Mutex<Responses>>,
    pub components: Arc<Mutex<ComponentHolder>>,
    pub context: ApplicationState,
}

impl Default for UiData {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
            responses: Arc::new(Mutex::new(Responses::new())),
            components: Arc::new(Mutex::new(ComponentHolder::default())),
            context: ApplicationState::default(),
        }
    }
}

impl UiData {
    pub fn get_components_mut(&mut self) -> MutexGuard<ComponentHolder> {
        self.components.lock().unwrap()
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }
}

pub struct Component {
    pub boundary: Option<Boundary>,
    pub enabled: bool,
}

impl Component {
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

impl Default for Component {
    fn default() -> Self {
        Self {
            boundary: None,
            enabled: true,
        }
    }
}

#[derive(Default)]
pub struct ComponentHolder {
    pub menubar: Component,
    pub taskbar: Component,
    pub modebar: Component,
    pub toolbar: Component,
    pub settingsbar: Component,
    pub addons: Component,
}

impl ComponentHolder {
    pub fn delete_cache(&mut self) {
        self.menubar.delete_cache();
        self.taskbar.delete_cache();
        self.modebar.delete_cache();
        self.toolbar.delete_cache();
        self.settingsbar.delete_cache();
    }
}
