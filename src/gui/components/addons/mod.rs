/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use bevy_egui::egui::{self, *};
use egui_extras::{Size, StripBuilder};

use crate::{
    gui::{self, Boundary, UiData},
    view::Mode,
};

mod force_analytics;
mod prepare;
mod preview;

type AddonStripBuilderClosure = dyn Fn(StripBuilder, UiData, Color32);

pub fn create_addon_strip_builder(
    ui: &mut Ui,
    data: UiData,
    boundary: Boundary,
    shaded_color: Color32,
    build: Box<AddonStripBuilderClosure>,
) -> Response {
    StripBuilder::new(ui)
        .size(Size::exact(boundary.location.x))
        .size(Size::exact(boundary.size.x))
        .size(Size::remainder())
        .horizontal(|mut strip| {
            strip.empty();
            strip.strip(|builder| {
                builder
                    .size(Size::exact(boundary.location.y))
                    .size(Size::exact(boundary.size.y))
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
    use bevy_egui::egui::{self, *};
    use egui_extras::Size;
    use egui_grid::GridBuilder;

    use crate::gui::{icon, UiData};

    pub fn show(ui: &mut Ui, data: UiData) {
        let layout = egui::Layout {
            main_dir: Direction::RightToLeft,
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
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::remainder())
            .show(ui, |mut grid| {
                grid.empty();
                grid.cell(|ui| {
                    let icon =
                        icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Diagonal);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {}
                });

                grid.cell(|ui| {
                    let icon =
                        icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Front);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {}
                });

                grid.cell(|ui| {
                    let icon = icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Top);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {}
                });

                grid.cell(|ui| {
                    let icon = icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Left);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {}
                });

                grid.cell(|ui| {
                    let icon =
                        icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Right);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {}
                });

                grid.empty();
            });
    }
}

pub struct Addons {}

impl Addons {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::InnerComponent<Addons> for Addons {
    fn show(&mut self, ctx: &egui::Context, ui: &mut Ui, data: UiData) {
        let window_size = ui.available_size();

        let boundary = Boundary {
            location: Pos2::new(0.0, 4.0),
            size: Vec2::new(window_size.x - 15.0, window_size.y - 15.0),
        };

        match data.mode {
            Mode::Prepare => prepare::show(ctx, ui, data, boundary),
            Mode::Preview => preview::show(ctx, ui, data, boundary),
            Mode::ForceAnalytics => force_analytics::show(ctx, ui, data, boundary),
        }
    }
}
