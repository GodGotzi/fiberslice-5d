use egui_extras::Size;
use egui_grid::GridBuilder;
use three_d::egui;

use crate::application::Application;
use crate::config;
use crate::prelude::*;

use super::Component;

pub struct Modebar;

impl Modebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component<Modebar> for Modebar {
    fn show(&mut self, ctx: &egui::Context, app: &mut Application) {
        egui::TopBottomPanel::bottom("modebar")
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
                        .cell(Size::initial(5.0))
                        .cell(Size::remainder())
                        .cell(Size::initial(5.0))
                        .cell(Size::remainder())
                        .cell(Size::initial(5.0))
                        .cell(Size::remainder())
                        .cell(Size::initial(5.0))
                        .show(ui, |mut grid| {
                            // Cells are represented as they were allocated
                            grid.cell(|ui| {
                                ui.selectable_value(app.mode(), Mode::Prepare, "Prepare");
                            });
                            grid.cell(|ui| {
                                ui.horizontal(|ui| {
                                    ui.separator();
                                });
                            });
                            grid.cell(|ui| {
                                ui.selectable_value(
                                    app.mode(),
                                    Mode::ForceAnalytics,
                                    "Force - Analytics",
                                );
                            });
                            grid.cell(|ui| {
                                ui.horizontal(|ui| {
                                    ui.separator();
                                });
                            });
                            grid.cell(|ui| {
                                ui.selectable_value(app.mode(), Mode::Preview, "Preview");
                            });
                            grid.cell(|ui| {
                                ui.horizontal(|ui| {
                                    ui.separator();
                                });
                            });
                            grid.cell(|ui| {
                                ui.selectable_value(app.mode(), Mode::Monitor, "Monitor");
                            });
                        });
                });
            });
    }
}
