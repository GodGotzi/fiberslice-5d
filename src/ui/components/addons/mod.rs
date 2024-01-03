/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use egui_extras::{Size, StripBuilder};
use three_d::egui::{self, *};

use crate::{
    ui::{boundary::Boundary, InnerComponent, UiData},
    view::Mode,
};

mod force_analytics;
mod prepare;
mod preview;

type AddonStripBuilderClosure = dyn Fn(StripBuilder, &mut UiData, Color32);

pub fn create_addon_strip_builder(
    ui: &mut Ui,
    data: &mut UiData,
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
    use egui_extras::Size;
    use egui_grid::GridBuilder;
    use strum::{EnumCount, IntoEnumIterator};
    use three_d::egui;

    use crate::{
        config,
        ui::{api::buttons::DecoradedButtons, UiData},
        view::Orientation,
    };

    pub fn show(ui: &mut egui::Ui, data: &mut UiData) {
        let layout = egui::Layout {
            main_dir: egui::Direction::RightToLeft,
            main_wrap: true,
            main_align: egui::Align::Center,
            main_justify: false,
            cross_align: egui::Align::Center,
            cross_justify: true,
        };

        //skip first because first is Orientation::Default we don't want that
        let builder = (1..Orientation::COUNT).fold(
            GridBuilder::new()
                .new_row_align(Size::remainder(), egui::Align::Center)
                .layout_standard(layout)
                .clip(true)
                .cell(Size::remainder()),
            |builder, _| builder.cell(Size::initial(40.0)),
        );

        builder.cell(Size::remainder()).show(ui, |mut grid| {
            grid.empty();

            //skip first because first is Orientation::Default we don't want that
            Orientation::iter().skip(1).for_each(|orientation| {
                grid.cell(|ui| {
                    ui.add_responsive_button(
                        data,
                        &config::gui::ORIENATION_BUTTON,
                        orientation,
                        &|data| {
                            data.borrow_shared_state().writer_environment_event.send(
                                crate::environment::EnvironmentEvent::SendOrientation(orientation),
                            )
                        },
                    )
                });
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

impl InnerComponent for Addons {
    fn show(&mut self, ctx: &egui::Context, ui: &mut Ui, state: &mut UiData) {
        let window_size = ui.available_size();

        let boundary = Boundary::new(
            Pos2::new(0.0, 4.0),
            Vec2::new(window_size.x - 15.0, window_size.y - 15.0),
        );

        let mode = state.borrow_ui_state().mode;

        match mode {
            Mode::Prepare => prepare::show(ctx, ui, state, boundary),
            Mode::Preview => preview::show(ctx, ui, state, boundary),
            Mode::ForceAnalytics => force_analytics::show(ctx, ui, state, boundary),
        }
    }
}
