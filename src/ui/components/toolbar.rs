use bevy_egui::egui::{Context, SidePanel};

use crate::config;
use crate::ui::{Component, UiData};

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Toolbar {
    fn show(&mut self, ctx: &Context, data: &mut UiData) {
        let boundary = SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |_ui| {})
            .response
            .into();

        data.context
            .get_component_data_mut()
            .toolbar
            .set_boundary(boundary);
    }
}
