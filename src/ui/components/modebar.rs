use bevy_egui::egui;
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::config;
use crate::ui::{Component, UiData};
use crate::view::Mode;

pub struct Modebar;

impl Modebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Modebar {
    fn show(&mut self, ctx: &egui::Context, data: &mut UiData) {
        let boundary = egui::TopBottomPanel::bottom("modebar")
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
                                ui.selectable_value(&mut data.mode, Mode::Prepare, "Prepare");
                            });
                            grid.empty();
                            grid.cell(|ui| {
                                ui.selectable_value(
                                    &mut data.mode,
                                    Mode::ForceAnalytics,
                                    "Force - Analytics",
                                );
                            });
                            grid.empty();
                            grid.cell(|ui| {
                                ui.selectable_value(&mut data.mode, Mode::Preview, "Preview");
                            });
                        });
                });
            })
            .response
            .into();

        data.get_components_mut().menubar.set_boundary(boundary);
    }
}
