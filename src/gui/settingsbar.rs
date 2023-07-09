/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::prelude::ResMut;
use bevy_egui::egui;
use egui::{Context, Direction, Ui};
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::{prelude::*, config};


#[derive(PartialEq)]
pub enum SettingsPanel {
    Slice,
    Filament,
    Printer,
}

struct TabbedSettings;

impl TabbedSettings {
    pub fn init() -> Self {
        Self {
        }
    }

    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui, side_view: &mut Settingsbar) {
        ui.horizontal(|ui| {
            let layout = egui::Layout {
                main_dir: Direction::TopDown,
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
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::Slice, "Slicer");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::Filament, "Filament");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::Printer, "Printer");
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

        ui.add_space(20.0 );

        match side_view.open_panel {
            SettingsPanel::Slice => {
                ui.label("a");
            },
            SettingsPanel::Filament => {
                ui.label("b");
            },
            SettingsPanel::Printer => {
                ui.label("c");
            },
        }
    }
}

pub struct Settingsbar {
    open_panel: SettingsPanel
}

impl Settingsbar {
    pub fn new() -> Self {
        Self {
            open_panel: SettingsPanel::Slice
        }
    }
}

impl super::Component<Settingsbar> for Settingsbar {

    fn show(&mut self, ctx: &egui::Context, 
        _ui: Option<&mut Ui>,
        _mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<super::Interface>,          
        item_wrapper: &mut ResMut<AsyncWrapper>, 
    ) {
        let mut tabbed_view = TabbedSettings::init();

        let response = egui::SidePanel::right("settingsbar")
            .resizable(true)
            .default_width(config::gui::default::SETTINGSBAR_W)
            .show(ctx, |ui| {
                
                item_wrapper.register(Item::SettingsWidth(Some(ui.available_width())));
 
                tabbed_view.show(ctx, ui, self);
                
            }).response;

        let rect = response.rect;

        gui_interface.register_boundary(
            super::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }
}
