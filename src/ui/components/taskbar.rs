use crate::config;
use crate::ui::boundary::Boundary;
use crate::ui::{Component, Theme, UiData};

pub struct Taskbar {
    boundary: Boundary,
    enabled: bool,
}

impl Taskbar {
    pub fn new() -> Self {
        Self {
            boundary: Boundary::zero(),
            enabled: true,
        }
    }
}

impl Component for Taskbar {
    fn show(&mut self, ctx: &egui::Context, data: &mut UiData) {
        self.boundary = egui::TopBottomPanel::bottom("taskbar")
            .default_height(config::gui::TASKBAR_H)
            .show(ctx, |ui: &mut egui::Ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.label(format!("{:.3} ms", data.global.ctx.frame_time * 1000.0));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme_button(ui, data);
                    });
                });
            })
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

fn theme_button(ui: &mut egui::Ui, data: &mut UiData) {
    let clicked = match data.borrow_ui_state().theme {
        Theme::Dark => ui.button("💡").clicked(),
        Theme::Light => ui.button("🌙").clicked(),
    };

    if clicked {
        data.borrow_mut_ui_state().toggle_theme();
    }
}
