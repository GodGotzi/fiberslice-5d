/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use egui::*;
use egui_xml::load_layout;
use orientation::OrientationAddon;

use crate::config::gui::shaded_color;
use crate::ui::InnerComponent;
use crate::ui::{AllocateInnerUiRect, UiState};
use crate::{GlobalState, RootEvent};

pub mod orientation {
    use egui::{Color32, ImageButton, Widget};
    use egui_extras::Size;
    use egui_grid::GridBuilder;
    use strum::{EnumCount, IntoEnumIterator};

    use crate::{
        config::{self, gui::shaded_color},
        render::RenderEvent,
        ui::{icon::get_orientation_asset, UiState},
        GlobalState, RootEvent,
    };

    use crate::environment::view::Orientation;

    pub struct OrientationAddon<'a> {
        shared_state: &'a (UiState, GlobalState<RootEvent>),
    }

    impl Widget for OrientationAddon<'_> {
        fn ui(self, ui: &mut egui::Ui) -> egui::Response {
            let (_ui_state, global_state) = self.shared_state;

            let layout = egui::Layout {
                main_dir: egui::Direction::RightToLeft,
                main_wrap: true,
                main_align: egui::Align::Center,
                main_justify: false,
                cross_align: egui::Align::Center,
                cross_justify: true,
            };

            let shaded_color = shaded_color(ui.visuals().dark_mode);

            ui.painter()
                .rect_filled(ui.available_rect_before_wrap(), 5.0, shaded_color);

            //skip first because first is Orientation::Default we don't want that
            let builder = (1..Orientation::COUNT).fold(
                GridBuilder::new()
                    .new_row_align(Size::remainder(), egui::Align::Center)
                    .layout_standard(layout)
                    .clip(true)
                    .cell(Size::remainder()),
                |builder, _| builder.cell(Size::initial(40.0)),
            );

            let before = ui.visuals_mut().widgets.inactive.weak_bg_fill;
            ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

            let response = builder.cell(Size::remainder()).show(ui, |mut grid| {
                grid.empty();

                //skip first because first is Orientation::Default we don't want that
                Orientation::iter().skip(1).for_each(|orientation| {
                    grid.cell(|ui| {
                        let button = config::gui::ORIENATION_BUTTON;

                        let icon = get_orientation_asset(orientation);

                        let image_button = ImageButton::new(icon).frame(true);

                        ui.allocate_ui(
                            [button.size.0 + button.border, button.size.1 + button.border].into(),
                            |ui| {
                                let response =
                                    ui.add_sized([button.size.0, button.size.1], image_button);

                                if response.clicked() {
                                    global_state
                                        .proxy
                                        .send_event(crate::RootEvent::RenderEvent(
                                            RenderEvent::CameraOrientationChanged(orientation),
                                        ))
                                        .unwrap();
                                }
                            },
                        );
                    });
                });
                grid.empty();
            });

            ui.visuals_mut().widgets.inactive.weak_bg_fill = before;

            response
        }
    }

    impl<'a> OrientationAddon<'a> {
        pub fn new(shared_state: &'a (UiState, GlobalState<RootEvent>)) -> Self {
            Self { shared_state }
        }
    }
}

pub struct Addons {
    enabled: bool,
}

impl Addons {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    fn show_orientation(&mut self, ui: &mut Ui, shared_state: &(UiState, GlobalState<RootEvent>)) {
        ui.add(OrientationAddon::new(shared_state));
    }

    fn show_bottom_addon(
        &mut self,
        ui: &mut Ui,
        (ui_state, _global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        let shaded_color = shaded_color(ui.visuals().dark_mode);

        ui_state.mode.read_with_fn(|mode| match mode {
            crate::environment::view::Mode::Preview => {
                ui.allocate_ui_in_rect(
                    Rect::from_two_pos(
                        Pos2::new(ui.available_width() * 0.25, 0.0),
                        Pos2::new(ui.available_width() * 0.75, ui.available_height()),
                    ),
                    |ui| {
                        ui_state.time_stamp.write_with_fn(|time_stamp| {
                            ui.spacing_mut().slider_width = ui.available_width();

                            let slider = egui::Slider::new(time_stamp, 0..=120)
                                .orientation(egui::SliderOrientation::Horizontal);
                            ui.add_sized(ui.available_size(), slider);
                        });
                    },
                );
            }
            crate::environment::view::Mode::Prepare => {}
            crate::environment::view::Mode::ForceAnalytics => {
                ui.allocate_ui_in_rect(
                    Rect::from_two_pos(
                        Pos2::new(ui.available_width() * 0.25, 0.0),
                        Pos2::new(ui.available_width() * 0.75, ui.available_height()),
                    ),
                    |ui| {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            5.0,
                            shaded_color,
                        );
                    },
                );
            }
        });
    }

    fn show_right_addon(
        &mut self,
        ui: &mut Ui,
        (ui_state, _global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        ui_state.mode.read_with_fn(|mode| match mode {
            crate::environment::view::Mode::Preview => {
                ui.allocate_ui_in_rect(
                    Rect::from_two_pos(
                        Pos2::new(0.0, ui.available_height() * 0.25),
                        Pos2::new(ui.available_width(), ui.available_height() * 0.75),
                    ),
                    |ui| {
                        ui_state.layer_max.write_with_fn(|layer_max| {
                            ui.spacing_mut().slider_width = ui.available_height();

                            let slider = egui::Slider::new(layer_max, 0..=120)
                                .orientation(egui::SliderOrientation::Vertical);
                            ui.add_sized(ui.available_size(), slider);
                        });
                    },
                );
            }
            crate::environment::view::Mode::Prepare => {}
            crate::environment::view::Mode::ForceAnalytics => {}
        });
    }

    fn show_left_addon(
        &mut self,
        ui: &mut Ui,
        (ui_state, _global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        let shaded_color = shaded_color(ui.visuals().dark_mode);

        ui_state.mode.read_with_fn(|mode| match mode {
            crate::environment::view::Mode::Preview => {}
            crate::environment::view::Mode::Prepare => {
                ui.allocate_ui_in_rect(
                    Rect::from_two_pos(
                        Pos2::new(0.0, ui.available_height() * 0.25),
                        Pos2::new(ui.available_width(), ui.available_height() * 0.75),
                    ),
                    |ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 5.0, shaded_color)
                    },
                );
            }
            crate::environment::view::Mode::ForceAnalytics => {
                ui.allocate_ui_in_rect(
                    Rect::from_two_pos(
                        Pos2::new(0.0, ui.available_height() * 0.25),
                        Pos2::new(ui.available_width(), ui.available_height() * 0.75),
                    ),
                    |ui| {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            5.0,
                            shaded_color,
                        );
                    },
                );
            }
        });
    }
}

impl InnerComponent for Addons {
    fn show(&mut self, ui: &mut Ui, shared_state: &(UiState, GlobalState<RootEvent>)) {
        if self.enabled {
            let available_size = ui.available_size();

            load_layout!(
                <Strip direction="north">
                    <Panel size="exact" value="50">
                        <Strip direction="west">
                            <Panel size="remainder"></Panel>
                            <Panel size="exact" value="240">
                                if available_size.x >= 240.0 {
                                    self.show_orientation(ui, shared_state);
                                }
                            </Panel>
                        </Strip>
                    </Panel>
                    <Panel size="remainder">
                        <Strip direction="west">
                            <Panel size="exact" value="80">
                                if available_size.x >= 80.0 {
                                    self.show_left_addon(ui, shared_state);
                                }
                            </Panel>
                            <Panel size="remainder"></Panel>
                            <Panel size="exact" value="50">
                                if available_size.x >= 50.0 {
                                    self.show_right_addon(ui, shared_state);
                                }
                            </Panel>
                        </Strip>
                    </Panel>
                    <Panel size="exact" value="80">
                        if available_size.y >= 80.0 {
                            self.show_bottom_addon(ui, shared_state);
                        }
                    </Panel>
                </Strip>
            );
        }
    }

    fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}
