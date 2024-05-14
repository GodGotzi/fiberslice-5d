use egui::{Context, SidePanel};
use three_d::egui;

use crate::config;
use crate::ui::boundary::Boundary;
use crate::ui::Component;
use crate::ui::UiData;

#[derive(Debug, Clone)]
pub struct Toolbar {
    boundary: Boundary,
    enabled: bool,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            boundary: Boundary::zero(),
            enabled: true,
        }
    }
}

impl Component for Toolbar {
    fn show(&mut self, ctx: &Context, data: &mut UiData) {
        self.boundary = SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |_ui| {})
            .response
            .into();
    }

    fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_boundary(&self) -> &Boundary {
        &self.boundary
    }
}
