use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::environment::view::Mode;
use crate::prelude::UnparallelSharedMut;
use crate::ui::boundary::Boundary;
use crate::ui::{Component, UiState};
use crate::{config, GlobalState, RootEvent};

pub struct Modebar {
    boundary: Boundary,
    enabled: UnparallelSharedMut<bool>,
}

impl Modebar {
    pub fn new() -> Self {
        Self {
            boundary: Boundary::zero(),
            enabled: UnparallelSharedMut::from_inner(true),
        }
    }
}

impl Component for Modebar {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (ui_state, _global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        if *self.enabled.inner().borrow() {
            self.boundary = egui::TopBottomPanel::bottom("modebar")
                .default_height(config::gui::MODEBAR_H)
                .show(ctx, |ui: &mut egui::Ui| {
                    egui::menu::bar(ui, |ui| {
                        let layout = egui::Layout {
                            main_dir: egui::Direction::TopDown,
                            main_wrap: false,
                            main_align: egui::Align::Center,
                            main_justify: false,
                            cross_align: egui::Align::Center,
                            cross_justify: true,
                        };

                        GridBuilder::new()
                            // Allocate a new row
                            .new_row_align(Size::initial(17.0), egui::Align::Center)
                            // Give this row a couple cells
                            .layout_standard(layout)
                            .clip(true)
                            .cell(Size::remainder())
                            .cell(Size::initial(-13.0))
                            .cell(Size::remainder())
                            .cell(Size::initial(-13.0))
                            .cell(Size::remainder())
                            .show(ui, |mut grid| {
                                // Cells are represented as they were allocated
                                grid.cell(|ui| {
                                    ui_state.mode.write_with_fn(|mode| {
                                        ui.selectable_value(mode, Mode::Prepare, "Prepare");
                                    });
                                });
                                grid.empty();
                                grid.cell(|ui| {
                                    ui_state.mode.write_with_fn(|mode| {
                                        ui.selectable_value(
                                            mode,
                                            Mode::ForceAnalytics,
                                            "Force - Analytics",
                                        );
                                    });
                                });
                                grid.empty();
                                grid.cell(|ui| {
                                    ui_state.mode.write_with_fn(|mode| {
                                        ui.selectable_value(mode, Mode::Preview, "Preview");
                                    });
                                });
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
