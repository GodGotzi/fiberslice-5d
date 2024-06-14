use crate::{GlobalState, RootEvent};

use super::UiState;

#[derive(Debug, Default)]
pub struct Windows {
    camera: CameraWindow,
}

impl Windows {
    pub fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        self.camera.show(ctx, shared_state);
    }
}

#[derive(Debug, Default)]
pub struct CameraWindow {
    enabled: bool,
}

impl CameraWindow {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        (ui_state, global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        egui::Window::new("Camera")
            .enabled(self.enabled)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.label("Camera settings");
            });
    }
}
