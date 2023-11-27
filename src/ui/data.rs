use std::sync::{Arc, Mutex, MutexGuard};

use three_d::FrameInput;

use crate::view::Mode;

use super::{boundary::Boundary, Theme};

pub struct UiData {
    pub theme: Theme,
    pub mode: Mode,

    pub context: ApplicationContext,
}

impl Default for UiData {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
            context: ApplicationContext::default(),
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

#[derive(Clone)]
pub struct ApplicationContext {
    pub frame_input: Option<FrameInput>,
    component_data: Arc<Mutex<ComponentDataHolder>>,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        Self {
            frame_input: None,
            component_data: Arc::new(Mutex::new(ComponentDataHolder::default())),
        }
    }
}

impl ApplicationContext {
    pub fn get_component_data_mut(&self) -> MutexGuard<ComponentDataHolder> {
        self.component_data.lock().unwrap()
    }

    pub fn fps(&self) -> Option<f32> {
        if let Some(frame_input) = &self.frame_input {
            Some((1000.0 / frame_input.elapsed_time) as f32)
        } else {
            None
        }
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
