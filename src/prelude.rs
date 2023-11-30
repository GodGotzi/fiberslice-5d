use std::sync::{Arc, Mutex};

use three_d::FrameInput;

pub use crate::error::Error;
use crate::{
    api::FrameHandle, settings::FilamentSettings, settings::PrinterSettings,
    settings::SliceSettings,
};

type Shared<T> = Arc<Mutex<T>>;

pub struct SharedState {
    inner: Shared<ApplicationState>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ApplicationState::default())),
        }
    }

    pub fn lock(&self) -> std::sync::MutexGuard<ApplicationState> {
        self.inner.lock().expect("Failed to lock shared state")
    }

    pub fn fps(&self) -> Option<f32> {
        self.lock().fps()
    }
}

#[derive(Default)]
pub struct ApplicationSettings {
    pub slice_settings: SliceSettings,
    pub printer_settings: PrinterSettings,
    pub filament_settings: FilamentSettings,
}

#[derive(Default)]
pub struct ApplicationState {
    frame_input: Option<FrameInput>,
    pub settings: ApplicationSettings,
}

impl ApplicationState {
    pub fn fps(&self) -> Option<f32> {
        if let Some(frame_input) = &self.frame_input {
            Some((1000.0 / frame_input.elapsed_time) as f32)
        } else {
            None
        }
    }
}

impl FrameHandle for ApplicationState {
    fn handle_frame(&mut self, frame_input: &FrameInput) {
        self.frame_input = Some(frame_input.clone());
    }
}
