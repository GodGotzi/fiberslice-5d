use bevy_egui::egui;
use egui::{Context, Direction, Ui};
use egui_extras::Size;
use egui_grid::GridBuilder;

#[derive(PartialEq)]
pub enum OptionPanel {
    SliceSettings,
    FilamentSettings,
    PrinterSettings,
}

struct TabbedView {
}

impl TabbedView {
    pub fn init() -> Self {
        Self {
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui, side_view: &mut SideView) {
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
                        ui.selectable_value(&mut side_view.open_panel, OptionPanel::SliceSettings, "Slice Settings");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, OptionPanel::FilamentSettings, "Filament Settings");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, OptionPanel::PrinterSettings, "Printer Settings");
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != OptionPanel::SliceSettings {
                                ui.separator();
                            }
                        });
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != OptionPanel::FilamentSettings {
                                ui.separator();
                            }
                        });
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != OptionPanel::PrinterSettings {
                                ui.separator();
                            }
                        });
                    });
                });
        });

        ui.add_space(20.0 );

        match side_view.open_panel {
            OptionPanel::SliceSettings => {
                ui.label("a");
            },
            OptionPanel::FilamentSettings => {
                ui.label("b");
            },
            OptionPanel::PrinterSettings => {
                ui.label("c");
            },
            _ => {

            }
        }
    }
}

pub(crate) struct SideView {
    open_panel: OptionPanel,
}


impl SideView {
    pub fn init() -> SideView {
        SideView {
            open_panel: OptionPanel::SliceSettings,
        }
    }

    pub fn side_panel_ui(&mut self, ctx: &Context) {
        let mut tabbed_view = TabbedView::init();

        egui::SidePanel::right("settings-panel")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                tabbed_view.show(ctx, ui, self);
            });
    }
}