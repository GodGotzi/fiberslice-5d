use bevy::prelude::{ResMut, EventWriter};
use bevy_egui::egui;
use egui::{Context, Direction, Ui};
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::view::ViewInterface;

use super::GuiResizeEvent;

#[derive(PartialEq)]
pub enum SettingsPanel {
    Slice,
    Filament,
    Printer,
}

struct TabbedView {
}

impl TabbedView {
    pub fn init() -> Self {
        Self {
        }
    }

    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui, side_view: &mut SideView, view_interface: &mut ResMut<ViewInterface>) {
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
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::Slice, "Slice Settings");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::Filament, "Filament Settings");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::Printer, "Printer Settings");
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

                if ui.button("test").clicked() {
                    view_interface.change_view_color(0.2, 0.3, 0.4);
                }

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

pub(crate) struct SideView {
    open_panel: SettingsPanel,
}


impl SideView {
    pub fn init() -> SideView {
        SideView {
            open_panel: SettingsPanel::Slice,
        }
    }

    pub fn side_panel_ui(&mut self, ctx: &Context, view_interface: &mut ResMut<ViewInterface>, events: &mut EventWriter<GuiResizeEvent>) {
        let mut tabbed_view = TabbedView::init();

        egui::SidePanel::right("settings-panel")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                events.send(GuiResizeEvent::Side(ui.available_width()));
                view_interface.diff_width_side = ui.available_width() as u32 + 1;
                tabbed_view.show(ctx, ui, self, view_interface);
            });
    }
}