/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use bevy_egui::egui;
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::config;

use crate::gui::*;

#[derive(PartialEq)]
pub enum SettingsPanel {
    Slice,
    Filament,
    Printer,
}

struct TabbedSettings;

impl TabbedSettings {
    pub fn init() -> Self {
        Self {}
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        data: UiData,
        side_view: &mut Settingsbar,
    ) {
        ui.horizontal(|ui| {
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
                .new_row_align(Size::initial(5.0), egui::Align::Center)
                .cell(Size::remainder())
                .cell(Size::remainder())
                .cell(Size::remainder())
                .show(ui, |mut grid| {
                    // Cells are represented as they were allocated
                    grid.cell(|ui| {
                        ui.selectable_value(
                            &mut side_view.open_panel,
                            SettingsPanel::Slice,
                            "Slicer",
                        );
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(
                            &mut side_view.open_panel,
                            SettingsPanel::Filament,
                            "Filament",
                        );
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(
                            &mut side_view.open_panel,
                            SettingsPanel::Printer,
                            "Printer",
                        );
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != SettingsPanel::Slice {
                                ui.separator();
                            }
                        });
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != SettingsPanel::Filament {
                                ui.separator();
                            }
                        });
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != SettingsPanel::Printer {
                                ui.separator();
                            }
                        });
                    });
                });
        });

        //ui.add_space(20.0);

        match side_view.open_panel {
            SettingsPanel::Slice => {
                egui::SidePanel::left("slice_settings_navigate")
                    .exact_width(30.0)
                    .resizable(false)
                    .show_inside(ui, |_ui| {});
            }
            SettingsPanel::Filament => {
                egui::SidePanel::left("filament_settings_navigate")
                    .exact_width(30.0)
                    .resizable(false)
                    .show_inside(ui, |_ui| {});
            }
            SettingsPanel::Printer => {
                let mut printer_settings = data.settings.printer.borrow_mut();

                egui::SidePanel::left("printer_settings_navigate")
                    .exact_width(30.0)
                    .resizable(false)
                    .show_inside(ui, |_ui| {});

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        printer_settings.general.show(ctx, ui);
                        printer_settings.machine_limits.show(ctx, ui);
                        printer_settings.extruder.show(ctx, ui);
                    });
                });
            }
        }
    }
}

pub struct Settingsbar {
    open_panel: SettingsPanel,
}

impl Settingsbar {
    pub fn new() -> Self {
        Self {
            open_panel: SettingsPanel::Slice,
        }
    }
}

impl Component for Settingsbar {
    fn show(&mut self, ctx: &egui::Context, data: UiData) {
        let mut tabbed_view = TabbedSettings::init();

        let boundary = Boundary::from(
            egui::SidePanel::right("settingsbar")
                .resizable(true)
                .default_width(config::gui::default::SETTINGSBAR_W)
                .show(ctx, |ui| {
                    tabbed_view.show(ctx, ui, data, self);
                })
                .response,
        );

        data.raw
            .borrow_mut()
            .boundary_holder
            .set_settingsbar(boundary);
    }
}
