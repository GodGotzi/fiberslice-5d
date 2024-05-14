/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use three_d::egui;
use three_d::egui::*;

use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::config;
use crate::ui::boundary::Boundary;
use crate::ui::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsPanel {
    Fiber,
    TopologyOptimization,
    View,
}

struct TabbedSettings;

impl TabbedSettings {
    pub fn init() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, data: &mut UiData, side_view: &mut Settingsbar) {
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
                            SettingsPanel::Fiber,
                            "Fiber",
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
                            SettingsPanel::TopologyOptimization,
                            "Topology",
                        );
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(&mut side_view.open_panel, SettingsPanel::View, "View");
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != SettingsPanel::Fiber {
                                ui.separator();
                            }
                        });
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != SettingsPanel::TopologyOptimization {
                                ui.separator();
                            }
                        });
                    });
                    grid.cell(|ui| {
                        ui.vertical(|ui| {
                            if side_view.open_panel != SettingsPanel::View {
                                ui.separator();
                            }
                        });
                    });
                });
        });

        //ui.add_space(20.0);

        match side_view.open_panel {
            SettingsPanel::Fiber => {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            data.borrow_shared_state().settings.main.show(ui);
                        });
                    });
                });
            }
            SettingsPanel::TopologyOptimization => {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            data.borrow_shared_state().settings.main.show(ui);
                        });
                    });
                });
            }
            SettingsPanel::View => {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            //let now = Instant::now();
                            data.borrow_shared_state().settings.main.show(ui);
                            //println!("Tree Time: {:?}", now.elapsed());
                        });
                    });
                });
            }
        }

        ui.add_space(20.0);
    }
}

#[derive(Debug, Clone)]
pub struct Settingsbar {
    open_panel: SettingsPanel,

    boundary: Boundary,
    enabled: bool,
}

impl Settingsbar {
    pub fn new() -> Self {
        Self {
            open_panel: SettingsPanel::Fiber,

            boundary: Boundary::zero(),
            enabled: true,
        }
    }
}

impl Component for Settingsbar {
    fn show(&mut self, ctx: &egui::Context, data: &mut UiData) {
        let mut tabbed_view = TabbedSettings::init();

        self.boundary = Boundary::from(
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

                            let rich_text = RichText::new("Slice")
                                .color(Color32::BLACK)
                                .font(FontId::new(18.0, egui::FontFamily::Monospace));
                            let widget_text = widget_text::WidgetText::RichText(rich_text);

                            let slice_button = Button::new(widget_text)
                                .fill(ui.style().visuals.selection.bg_fill)
                                .min_size(Vec2::new(ui.available_width() * 0.8, 50.0));

                            if ui.add(slice_button).clicked() {
                                println!("{:?}", data.borrow_shared_state().settings.main);
                            };
                        });

                        ui.add_space(20.0);

                        ui.separator();

                        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                            tabbed_view.show(ui, data, self);
                        });
                    });
                })
                .response,
        );
    }

    fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_boundary(&self) -> &Boundary {
        &self.boundary
    }
}
