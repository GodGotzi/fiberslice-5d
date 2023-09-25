use bevy_egui::egui;

use crate::config;
use crate::gui;

use super::UiData;

pub struct Taskbar {}

impl Taskbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Taskbar> for Taskbar {
    fn show(&mut self, ctx: &egui::Context, mut data: UiData) {
        let boundary = egui::TopBottomPanel::bottom("taskbar")
            .default_height(config::gui::TASKBAR_H)
            .show(ctx, |ui: &mut egui::Ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        //ui.label(format!("{:.2} fps", data.context.lock().unwrap().fps()));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme_button(ui, data.reborrow());
                    });
                });
            })
            .response
            .into();

        data.reborrow().boundary_holder.set_taskbar(boundary);
    }
}

fn theme_button(ui: &mut egui::Ui, mut data: UiData) {
    let clicked = match data.theme {
        gui::Theme::Dark => ui.button("💡").clicked(),
        gui::Theme::Light => ui.button("🌙").clicked(),
    };

    if clicked {
        data.toggle_theme();
    }
}
