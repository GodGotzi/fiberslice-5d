use crate::prelude::UnparallelSharedMut;
use crate::ui::boundary::Boundary;
use crate::ui::{Component, Theme, UiState};
use crate::{config, GlobalState, RootEvent};

pub struct Taskbar {
    boundary: Boundary,
    enabled: UnparallelSharedMut<bool>,
}

impl Taskbar {
    pub fn new() -> Self {
        Self {
            boundary: Boundary::zero(),
            enabled: UnparallelSharedMut::from_inner(true),
        }
    }
}

impl Component for Taskbar {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (ui_state, global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        if *self.enabled.inner().borrow() {
            self.boundary = egui::TopBottomPanel::bottom("taskbar")
                .default_height(config::gui::TASKBAR_H)
                .show(ctx, |ui: &mut egui::Ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(10.0);
                            ui.label(format!("{:.3} ms", global_state.ctx.frame_time * 1000.0));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            theme_button(ui, ui_state);
                        });
                    });
                })
                .response
                .into();
        }
    }

    fn get_boundary(&self) -> &Boundary {
        &self.boundary
    }

    fn get_enabled(&self) -> UnparallelSharedMut<bool> {
        self.enabled.clone()
    }
}

fn theme_button(ui: &mut egui::Ui, ui_state: &UiState) {
    let clicked =
        ui_state
            .theme
            .read_with_fn(|theme| match theme.as_ref().expect("Theme not set") {
                Theme::Dark => ui.button("💡").clicked(),
                Theme::Light => ui.button("🌙").clicked(),
            });

    if clicked {
        ui_state.toggle_theme();
    }
}
