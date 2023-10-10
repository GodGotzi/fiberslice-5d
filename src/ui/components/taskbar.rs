use bevy_egui::egui;

use crate::config;
use crate::ui::{Component, Theme, UiData};

pub struct Taskbar {}

impl Taskbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Taskbar {
    fn show(&mut self, ctx: &egui::Context, data: UiData) {
        let boundary = egui::TopBottomPanel::bottom("taskbar")
            .default_height(config::gui::TASKBAR_H)
            .show(ctx, |ui: &mut egui::Ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.label(format!("{:.2} fps", data.context.fps()));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme_button(ui, data);
                    });
                });
            })
            .response
            .into();

        data.raw.borrow_mut().holder.taskbar.set_boundary(boundary);
    }
}

fn theme_button(ui: &mut egui::Ui, data: UiData) {
    let clicked = match data.raw.borrow_mut().theme {
        Theme::Dark => ui.button("💡").clicked(),
        Theme::Light => ui.button("🌙").clicked(),
    };

    if clicked {
        data.raw.borrow_mut().toggle_theme();
    }
}
