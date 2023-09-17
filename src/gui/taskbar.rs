use three_d::egui;

use crate::application::ApplicationContext;
use crate::config;
use crate::gui;

use super::GuiContext;

pub struct Taskbar {}

impl Taskbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Taskbar> for Taskbar {
    fn show(&mut self, ctx: &egui::Context, gui_context: &mut GuiContext) {
        let boundary = egui::TopBottomPanel::bottom("taskbar")
            .default_height(config::gui::TASKBAR_H)
            .show(ctx, |ui: &mut egui::Ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.label(format!("{:.2} fps", gui_context.application.context.fps()));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme_button(ui, &mut gui_context.application.context);
                    });
                });
            })
            .response
            .into();

        gui_context.application.context.boundaries_mut().taskbar = boundary;
    }
}

fn theme_button(ui: &mut egui::Ui, app: &mut ApplicationContext) {
    let clicked = match app.theme() {
        gui::Theme::Dark => ui.button("💡").clicked(),
        gui::Theme::Light => ui.button("🌙").clicked(),
    };

    if clicked {
        app.toggle_theme();
    }
}
