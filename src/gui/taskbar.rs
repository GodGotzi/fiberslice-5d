use three_d::egui;

use crate::application::Application;
use crate::gui;
use crate::{config, prelude::*};

pub struct Taskbar {}

impl Taskbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Taskbar> for Taskbar {
    fn show(&mut self, ctx: &egui::Context, app: &mut Application) {
        egui::TopBottomPanel::bottom("taskbar")
            .default_height(config::gui::TASKBAR_H)
            .show(ctx, |ui: &mut egui::Ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme_button(ui, app);
                    });
                });
            });
    }
}

fn theme_button(ui: &mut egui::Ui, app: &mut Application) {
    let clicked = match app.theme() {
        gui::Theme::Dark => ui.button("💡").clicked(),
        gui::Theme::Light => ui.button("🌙").clicked(),
    };

    if clicked {
        app.toggle_theme();
    }
}
