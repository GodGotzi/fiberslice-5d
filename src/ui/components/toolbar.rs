use std::rc::Rc;

use three_d::egui::{Context, SidePanel};

use crate::config;
use crate::ui::state::UiState;
use crate::ui::Component;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Toolbar {
    fn show(&mut self, ctx: &Context, mut data: Rc<UiState>) {
        let boundary = SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |_ui| {})
            .response
            .into();

        data.components.toolbar.set_boundary(boundary);
    }
}
