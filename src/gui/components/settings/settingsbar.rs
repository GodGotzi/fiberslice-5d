/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use bevy_egui::egui;
use bevy_egui::egui::Button;
use bevy_egui::egui::CollapsingHeader;

use bevy_egui::egui::Layout;
use bevy_egui::egui::Vec2;
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
                let mut printer_settings = data.settings.printer.borrow_mut();

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            CollapsingHeader::new("General")
                                .default_open(true)
                                .show(ui, |ui| {
                                    printer_settings.general.show(ctx, ui);
                                });

                            CollapsingHeader::new("Machine Limits")
                                .default_open(true)
                                .show(ui, |ui| {
                                    printer_settings.machine_limits.show(ctx, ui);
                                });

                            CollapsingHeader::new("Extruder")
                                .default_open(true)
                                .show(ui, |ui| {
                                    printer_settings.extruder.show(ctx, ui);
                                });
                        });
                    });
                });
            }
            SettingsPanel::Filament => {
                let mut filament_settings = data.settings.filament.borrow_mut();

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            CollapsingHeader::new("General")
                                .default_open(true)
                                .show(ui, |ui| {
                                    filament_settings.general.show(ctx, ui);
                                });

                            CollapsingHeader::new("Temperature")
                                .default_open(true)
                                .show(ui, |ui| {
                                    filament_settings.temperature.show(ctx, ui);
                                });

                            CollapsingHeader::new("Cooling")
                                .default_open(true)
                                .show(ui, |ui| {
                                    filament_settings.cooling.show(ctx, ui);
                                });

                            CollapsingHeader::new("Advanced")
                                .default_open(true)
                                .show(ui, |ui| {
                                    filament_settings.advanced.show(ctx, ui);
                                });
                        });
                    });
                });
            }
            SettingsPanel::Printer => {
                let mut printer_settings = data.settings.printer.borrow_mut();

                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            CollapsingHeader::new("General")
                                .default_open(true)
                                .show(ui, |ui| {
                                    printer_settings.general.show(ctx, ui);
                                });

                            CollapsingHeader::new("Machine Limits")
                                .default_open(true)
                                .show(ui, |ui| {
                                    printer_settings.machine_limits.show(ctx, ui);
                                });

                            CollapsingHeader::new("Extruder")
                                .default_open(true)
                                .show(ui, |ui| {
                                    printer_settings.extruder.show(ctx, ui);
                                });
                        });
                    });
                });
            }
        }

        ui.add_space(20.0);
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
                    ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(20.0);

                        ui.allocate_ui(Vec2::new(ui.available_width(), 250.0), |ui| {
                            let export_button = Button::new("Export GCode")
                                .min_size(Vec2::new(ui.available_width() * 0.5, 20.0));

                            ui.add_enabled(false, export_button);

                            let slice_button = Button::new("Slice")
                                .min_size(Vec2::new(ui.available_width() * 0.8, 50.0));

                            ui.add(slice_button);
                        });

                        ui.add_space(20.0);

                        ui.separator();

                        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                            tabbed_view.show(ctx, ui, data, self);
                        });
                    });
                })
                .response,
        );

        data.raw.borrow_mut().holder.set_settingsbar(boundary);
    }
}
