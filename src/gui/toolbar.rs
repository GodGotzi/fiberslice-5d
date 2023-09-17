use three_d::egui::{Context, SidePanel};

use crate::config;
use crate::prelude::*;

use super::Component;
use super::GuiContext;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component<Toolbar> for Toolbar {
    fn show(&mut self, ctx: &Context, gui_context: &mut GuiContext) {
        let boundary = SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |ui| {
                gui_context
                    .application
                    .context
                    .event_wrapping()
                    .register(Item::ToolbarWidth(Some(ui.available_width())));
            })
            .response
            .into();

        gui_context.application.context.boundaries_mut().toolbar = boundary;
    }
}
