use bevy_egui::egui::{self, Direction, Ui};
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::{utils::Creation, prelude::*};

pub struct Modebar;

impl Creation for Modebar {
    fn create() -> Self {
        Self { }
    }
}

impl super::Component<Modebar> for Modebar {

    fn show(&mut self, ctx: &bevy_egui::egui::Context,
        _ui: Option<&mut bevy_egui::egui::Ui>,
        mode_ctx: Option<&mut Mode>,
        gui_interface: &mut bevy::prelude::ResMut<super::Interface>,          
        _gui_events: &mut std::collections::HashMap<crate::prelude::ItemType, crate::prelude::AsyncPacket<crate::prelude::Item>>
    ) { 
        let mode = mode_ctx.unwrap();

        let response = egui::TopBottomPanel::bottom("modebar").show(ctx, |ui: &mut Ui| {
            egui::menu::bar(ui, |ui| {

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
                .cell(Size::initial(5.0))
                .cell(Size::remainder())
                .cell(Size::initial(5.0))
                .show(ui, |mut grid| {
                    // Cells are represented as they were allocated
                    grid.cell(|ui| {
                        ui.selectable_value(mode, Mode::Prepare, "Prepare");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(mode, Mode::ForceAnalytics, "Force - Analytics");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(mode, Mode::Preview, "Preview");
                    });
                    grid.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.separator();
                        });
                    });
                    grid.cell(|ui| {
                        ui.selectable_value(mode, Mode::Monitor, "Monitor");
                    });
                });
            });
        }).response;

        let rect = response.rect;

        gui_interface.register_boundary(
            super::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }

}