use std::sync::{Arc, Mutex};

use crate::{prelude::ApplicationState, view::Mode};

use super::{boundary::Boundary, response::Responses, Theme};

pub struct UiData {
    pub theme: Theme,
    pub mode: Mode,

    pub responses: Responses,
    pub components: ComponentHolder,
}

impl UiData {
    pub fn new(application_state: Arc<Mutex<ApplicationState>>) -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
            responses: Responses::new(),
            components: ComponentHolder::default(),
        }
    }
}

impl UiData {
    pub fn get_components(&self) -> &ComponentHolder {
        &self.components
    }

    pub fn get_components_mut(&mut self) -> &mut ComponentHolder {
        &mut self.components
    }

    pub fn responses(&self) -> &Responses {
        &self.responses
    }

    pub fn responses_mut(&mut self) -> &mut Responses {
        &mut self.responses
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
