use crate::{prelude::ApplicationState, view::Mode};

use super::{boundary::Boundary, Theme};

pub struct UiData {
    pub theme: Theme,
    pub mode: Mode,

    pub context: ApplicationState,
}

impl Default for UiData {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
            context: ApplicationState::default(),
        }
    }
}

impl UiData {
    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
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
