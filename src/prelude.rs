use three_d::FrameInput;

pub use crate::error::Error;
use crate::{settings::FilamentSettings, settings::PrinterSettings, settings::SliceSettings};

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

pub struct ApplicationSettings {
    pub slice_settings: SliceSettings,
    pub printer_settings: PrinterSettings,
    pub filament_settings: FilamentSettings,
}

pub struct ApplicationState {
    frame_input: Option<FrameInput>,
    pub settings: ApplicationSettings,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            frame_input: None,
            settings: ApplicationSettings {
                slice_settings: SliceSettings::default(),
                printer_settings: PrinterSettings::default(),
                filament_settings: FilamentSettings::default(),
            },
        }
    }
}

impl ApplicationState {
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
