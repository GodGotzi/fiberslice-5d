/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use egui_extras::{Size, StripBuilder};
use three_d::egui::{self, *};

use crate::{
    application::ApplicationContext,
    gui::{self, Boundary},
    view::Mode,
};

mod force_analytics;
mod monitor;
mod prepare;
mod preview;

type AddonStripBuilderClosure = dyn Fn(StripBuilder, &mut ApplicationContext, Color32);

pub fn create_addon_strip_builder(
    ui: &mut Ui,
    app: &mut ApplicationContext,
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
                            build(builder, app, shaded_color);
                        });
                        strip.empty();
                    });
            });
            strip.empty();
        })
}

pub mod orientation {
    use egui_extras::Size;
    use egui_grid::GridBuilder;
    use three_d::egui;
    use three_d::egui::*;

    use crate::{application::ApplicationContext, gui::icon};

    pub fn show(ui: &mut Ui, _app: &mut ApplicationContext) {
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
                        icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Default);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Default");
                    }
                });

                grid.cell(|ui| {
                    let icon =
                        icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Front);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Front");
                    }
                });

                grid.cell(|ui| {
                    let icon = icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Top);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Top");
                    }
                });

                grid.cell(|ui| {
                    let icon = icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Left);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Left");
                    }
                });

                grid.cell(|ui| {
                    let icon =
                        icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Right);

                    let image_button =
                        ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Right");
                    }
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
    fn show(&mut self, ctx: &egui::Context, ui: &mut Ui, app: &mut ApplicationContext) {
        let window_size = ui.available_size();

        let boundary = Boundary {
            location: Pos2::new(0.0, 4.0),
            size: Vec2::new(window_size.x - 15.0, window_size.y - 15.0),
        };

        match app.mode() {
            Mode::Prepare => prepare::show(ctx, ui, app, boundary),
            Mode::Preview => preview::show(ctx, ui, app, boundary),
            Mode::Monitor => monitor::show(ctx, ui, app, boundary),
            Mode::ForceAnalytics => force_analytics::show(ctx, ui, app, boundary),
        }
    }
}
