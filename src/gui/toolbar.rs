use bevy_egui::egui::{Context, SidePanel};

use crate::config;

use super::Component;
use super::UiData;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component<Toolbar> for Toolbar {
    fn show(&mut self, ctx: &Context, data: UiData) {
        let boundary = SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |_ui| {})
            .response
            .into();

        data.raw.borrow_mut().boundary_holder.set_toolbar(boundary);
    }
}
