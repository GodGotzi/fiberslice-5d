/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use std::rc::Rc;

use egui_extras::{Size, StripBuilder};
use three_d::egui::{self, *};

use crate::{
    ui::{boundary::Boundary, InnerComponent, UiState},
    view::Mode,
};

mod force_analytics;
mod prepare;
mod preview;

type AddonStripBuilderClosure = dyn Fn(StripBuilder, Rc<UiState>, Color32);

pub fn create_addon_strip_builder(
    ui: &mut Ui,
    data: Rc<UiState>,
    boundary: Boundary,
    shaded_color: Color32,
    build: Box<AddonStripBuilderClosure>,
) -> Response {
    StripBuilder::new(ui)
        .size(Size::exact(boundary.location.x))
        .size(Size::exact(boundary.get_width()))
        .size(Size::remainder())
        .horizontal(|mut strip| {
            strip.empty();
            strip.strip(|builder| {
                builder
                    .size(Size::exact(boundary.location.y))
                    .size(Size::exact(boundary.get_height()))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.empty();
                        strip.strip(|builder| {
                            build(builder, data, shaded_color);
                        });
                        strip.empty();
                    });
            });
            strip.empty();
        })
}

pub mod orientation {
    use std::rc::Rc;

    use egui_extras::Size;
    use egui_grid::GridBuilder;
    use three_d::egui::{self, ImageButton};

    use crate::{
        ui::{icon, response::Responsive, UiState},
        view::Orientation,
    };

    pub fn show(ui: &mut egui::Ui, data: Rc<UiState>) {
        let layout = egui::Layout {
            main_dir: egui::Direction::RightToLeft,
            main_wrap: true,
            main_align: egui::Align::Center,
            main_justify: false,
            cross_align: egui::Align::Center,
            cross_justify: false,
        };

        GridBuilder::new()
            // Allocate a new row
            .new_row_align(Size::remainder(), egui::Align::Center)
            // Give this row a couple cells
            .layout_standard(layout)
            .clip(true)
            .cell(Size::remainder())
            .cell(Size::initial(35.0))
            .cell(Size::initial(35.0))
            .cell(Size::initial(35.0))
            .cell(Size::initial(35.0))
            .cell(Size::initial(35.0))
            .cell(Size::remainder())
            .show(ui, |mut grid| {
                grid.empty();
                grid.cell(|ui| {
                    add_button_icon(ui, data, Orientation::Diagonal);
                });

                grid.cell(|ui| {
                    add_button_icon(ui, data, Orientation::Front);
                });

                grid.cell(|ui| {
                    add_button_icon(ui, data, Orientation::Top);
                });

                grid.cell(|ui| {
                    add_button_icon(ui, data, Orientation::Left);
                });

                grid.cell(|ui| {
                    add_button_icon(ui, data, Orientation::Right);
                });

                grid.empty();
            });
    }

    fn add_button_icon(ui: &mut egui::Ui, data: Rc<UiState>, orientation: Orientation) {
        let icon = icon::ICONTABLE.get_orientation_icon(orientation);

        let image_button =
            ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

        ui.allocate_ui([35., 35.].into(), move |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let prev_response = data.get_orientation_response(&orientation);

                    if prev_response.hovered() {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            egui::Color32::from_rgba_premultiplied(75, 255, 0, 100),
                        );
                    }

                    let response = ui.add_sized([30., 30.], image_button);

                    data.update_orientation_response(&orientation, response);

                    /*
                    if response.clicked() {
                        data.orienation_writer().borrow_mut().send(orientation);
                    }
                    */
                },
            );
        });
    }
}

pub struct Addons {}

impl Addons {
    pub fn new() -> Self {
        Self {}
    }
}

impl InnerComponent for Addons {
    fn show(&mut self, ctx: &egui::Context, ui: &mut Ui, state: Rc<UiState>) {
        let window_size = ui.available_size();

        let boundary = Boundary::new(
            Pos2::new(0.0, 4.0),
            Vec2::new(window_size.x - 15.0, window_size.y - 15.0),
        );

        match state.mode {
            Mode::Prepare => prepare::show(ctx, ui, state, boundary),
            Mode::Preview => preview::show(ctx, ui, state, boundary),
            Mode::ForceAnalytics => force_analytics::show(ctx, ui, state, boundary),
        }
    }
}
