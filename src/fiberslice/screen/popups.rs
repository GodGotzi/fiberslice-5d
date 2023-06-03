use bevy_egui::egui;
use egui::Context;

pub(crate) struct PopupsView {

}

impl PopupsView {
    pub fn init() -> PopupsView {
        PopupsView {
        }
    }

    pub fn popups_ui(&mut self, 
        ctx: &Context, 
    ) {

        egui::Window::new("Test")
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.label("Label test");
            let button = ui.button("button test");
        });
    }
}