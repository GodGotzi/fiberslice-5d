use three_d::egui::{Context, SidePanel};

use crate::application::Application;
use crate::config;
use crate::prelude::*;

use super::Component;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component<Toolbar> for Toolbar {
    fn show(&mut self, ctx: &Context, app: &mut Application) {
        SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |ui| {
                app.event_wrapping()
                    .register(Item::ToolbarWidth(Some(ui.available_width())));
            });
    }
}
