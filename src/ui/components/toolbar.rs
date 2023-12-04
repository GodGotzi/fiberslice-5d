use three_d::egui::{Context, SidePanel};

use crate::config;
use crate::ui::Component;
use crate::ui::UiData;

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

        data.borrow_mut_ui_state()
            .components
            .toolbar
            .set_boundary(boundary);
    }
}
