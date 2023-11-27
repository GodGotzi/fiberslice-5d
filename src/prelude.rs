use std::sync::{Arc, Mutex, MutexGuard};

use three_d::FrameInput;

pub use crate::error::Error;
use crate::{
    settings::FilamentSettings, settings::PrinterSettings, settings::SliceSettings,
    ui::data::ComponentDataHolder,
};

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct ApplicationSettings {
    slice_settings: SliceSettings,
    printer_settings: PrinterSettings,
    filament_settings: FilamentSettings,
}

#[derive(Clone)]
pub struct ApplicationState {
    frame_input: Option<FrameInput>,
    component_data: Arc<Mutex<ComponentDataHolder>>,
    settings: ApplicationSettings,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            frame_input: None,
            component_data: Arc::new(Mutex::new(ComponentDataHolder::default())),
        }
    }
}

impl ApplicationState {
    pub fn get_component_data_mut(&self) -> MutexGuard<ComponentDataHolder> {
        self.component_data.lock().unwrap()
    }

    pub fn update_frame_input(&mut self, frame_input: FrameInput) {
        self.frame_input = Some(frame_input);
    }

    pub fn fps(&self) -> Option<f32> {
        if let Some(frame_input) = &self.frame_input {
            Some((1000.0 / frame_input.elapsed_time) as f32)
        } else {
            None
        }
    }
}
