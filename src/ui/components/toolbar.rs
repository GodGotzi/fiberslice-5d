use egui_extras::Size;
use egui_grid::GridBuilder;
use three_d::egui::{self, Layout};
use three_d::egui::{Context, SidePanel};

use crate::config;
use crate::ui::api::buttons::DecoradedButtons;
use crate::ui::Component;
use crate::ui::UiData;
use crate::view::Orientation;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Toolbar {
    fn show(&mut self, ctx: &Context, data: &mut UiData) {
        let boundary = SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    GridBuilder::new()
                        // Allocate a new row
                        .new_row_align(Size::remainder(), egui::Align::Center)
                        // Give this row a couple cells
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
                                ui.add_responsive_button(
                                    data,
                                    &config::gui::ORIENATION_BUTTON,
                                    Orientation::Diagonal,
                                    Box::new(|data| {
                                        data.borrow_shared_state().writer_environment_event.send(
                                            crate::environment::EnvironmentEvent::SendOrientation(
                                                Orientation::Diagonal,
                                            ),
                                        )
                                    }),
                                )
                            });

                            grid.cell(|ui| {
                                ui.add_responsive_button(
                                    data,
                                    &config::gui::ORIENATION_BUTTON,
                                    Orientation::Front,
                                    Box::new(|data| {
                                        data.borrow_shared_state().writer_environment_event.send(
                                            crate::environment::EnvironmentEvent::SendOrientation(
                                                Orientation::Front,
                                            ),
                                        )
                                    }),
                                )
                            });

                            grid.cell(|ui| {
                                ui.add_responsive_button(
                                    data,
                                    &config::gui::ORIENATION_BUTTON,
                                    Orientation::Top,
                                    Box::new(|data| {
                                        data.borrow_shared_state().writer_environment_event.send(
                                            crate::environment::EnvironmentEvent::SendOrientation(
                                                Orientation::Top,
                                            ),
                                        )
                                    }),
                                )
                            });

                            grid.cell(|ui| {
                                ui.add_responsive_button(
                                    data,
                                    &config::gui::ORIENATION_BUTTON,
                                    Orientation::Left,
                                    Box::new(|data| {
                                        data.borrow_shared_state().writer_environment_event.send(
                                            crate::environment::EnvironmentEvent::SendOrientation(
                                                Orientation::Left,
                                            ),
                                        )
                                    }),
                                )
                            });

                            grid.cell(|ui| {
                                ui.add_responsive_button(
                                    data,
                                    &config::gui::ORIENATION_BUTTON,
                                    Orientation::Right,
                                    Box::new(|data| {
                                        data.borrow_shared_state().writer_environment_event.send(
                                            crate::environment::EnvironmentEvent::SendOrientation(
                                                Orientation::Right,
                                            ),
                                        )
                                    }),
                                )
                            });

                            grid.empty();
                        });
                });
            })
            .response
            .into();

        data.borrow_mut_ui_state()
            .components
            .toolbar
            .set_boundary(boundary);
    }
}
