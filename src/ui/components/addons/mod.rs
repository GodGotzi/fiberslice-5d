/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use crate::{ui::icon, view::Orientation};
use egui_extras::{Size, StripBuilder};
use strum::EnumCount;
use three_d::egui::{self, *};

use std::rc::Rc;

use crate::{
    api::ui::ResponsiveButton,
    ui::{boundary::Boundary, InnerComponent, UiData},
    view::Mode,
};

mod force_analytics;
mod prepare;
mod preview;

type AddonStripBuilderClosure = dyn Fn(Addons, StripBuilder, &mut UiData, Color32);

pub fn create_addon_strip_builder(
    addons: Addons,
    ui: &mut Ui,
    data: &mut UiData,
    boundary: Boundary,
    shaded_color: Color32,
    build: &AddonStripBuilderClosure,
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
                            build(addons, builder, data, shaded_color);
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
    use three_d::egui::{self, ImageButton};

    use crate::{
        ui::{icon, response::Responsive, UiData},
        view::Orientation,
    };

    pub fn show(addons: super::Addons, ui: &mut egui::Ui, data: &mut UiData) {
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
                    add_button_icon(addons.clone(), ui, data, Orientation::Diagonal);
                });

                grid.cell(|ui| {
                    add_button_icon(addons.clone(), ui, data, Orientation::Front);
                });

                grid.cell(|ui| {
                    add_button_icon(addons.clone(), ui, data, Orientation::Top);
                });

                grid.cell(|ui| {
                    add_button_icon(addons.clone(), ui, data, Orientation::Left);
                });

                grid.cell(|ui| {
                    add_button_icon(addons, ui, data, Orientation::Right);
                });

                grid.empty();
            });
    }

    fn add_button_icon(
        addons: super::Addons,
        ui: &mut egui::Ui,
        data: &mut UiData,
        orientation: Orientation,
    ) {
        if let Some(responsive_button) = addons.orientation {
            ui.allocate_ui([35., 35.].into(), move |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        let mut ui_state = data.state.borrow_mut();
                        let prev_response =
                            ui_state.responses.get_button_response(orientation).unwrap();

                        let response = responsive_button[orientation as usize]
                            .show(prev_response.hovered(), ui);

                        ui_state
                            .responses
                            .update_button_response(orientation, &response);

                        if response.clicked() {
                            println!("Clicked: {:?}", orientation);

                            data.borrow_shared_state().writer_environment_event.send(
                                crate::environment::EnvironmentEvent::SendOrientation(orientation),
                            )
                        }
                    },
                );
            });
        }
    }
}

pub struct Addons {
    orientation: Option<[Rc<ResponsiveButton>; Orientation::COUNT]>,
}

impl Clone for Addons {
    fn clone(&self) -> Self {
        if let Some(orientation) = &self.orientation {
            Self {
                orientation: Some([
                    orientation[0].clone(),
                    orientation[1].clone(),
                    orientation[2].clone(),
                    orientation[3].clone(),
                    orientation[4].clone(),
                    orientation[5].clone(),
                ]),
            }
        } else {
            Self { orientation: None }
        }
    }
}

impl Addons {
    pub fn new() -> Self {
        Self { orientation: None }
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
            Mode::Prepare => prepare::show(self.clone(), ctx, ui, state, boundary),
            Mode::Preview => preview::show(self.clone(), ctx, ui, state, boundary),
            Mode::ForceAnalytics => force_analytics::show(self.clone(), ctx, ui, state, boundary),
        }
    }

    fn init_with_ctx(&mut self, ctx: &egui::Context) {
        if self.orientation.is_some() {
            return;
        }

        let color = egui::Color32::from_rgba_premultiplied(75, 255, 0, 100);
        let icon_default = icon::ICONTABLE.get_orientation_icon(Orientation::Default);
        let icon_diagonal = icon::ICONTABLE.get_orientation_icon(Orientation::Diagonal);
        let icon_front = icon::ICONTABLE.get_orientation_icon(Orientation::Front);
        let icon_top = icon::ICONTABLE.get_orientation_icon(Orientation::Top);
        let icon_left = icon::ICONTABLE.get_orientation_icon(Orientation::Left);
        let icon_right = icon::ICONTABLE.get_orientation_icon(Orientation::Right);

        let orientation = [
            Rc::new(ResponsiveButton::new(
                icon_default,
                icon_default.texture_id(ctx),
                (30.0, 30.0),
                color,
            )),
            Rc::new(ResponsiveButton::new(
                icon_diagonal,
                icon_diagonal.texture_id(ctx),
                (30.0, 30.0),
                color,
            )),
            Rc::new(ResponsiveButton::new(
                icon_front,
                icon_front.texture_id(ctx),
                (30.0, 30.0),
                color,
            )),
            Rc::new(ResponsiveButton::new(
                icon_top,
                icon_top.texture_id(ctx),
                (30.0, 30.0),
                color,
            )),
            Rc::new(ResponsiveButton::new(
                icon_left,
                icon_left.texture_id(ctx),
                (30.0, 30.0),
                color,
            )),
            Rc::new(ResponsiveButton::new(
                icon_right,
                icon_right.texture_id(ctx),
                (30.0, 30.0),
                color,
            )),
        ];

        self.orientation = Some(orientation);
    }
}
