use std::collections::HashMap;

use bevy_egui::egui::{self, Ui};
use egui_extras::{StripBuilder, Size};

use crate::{prelude::*, gui};

pub fn show(
    _ctx: &egui::Context,
    ui_op: Option<&mut Ui>,
    _gui_interface: &mut bevy::prelude::ResMut<gui::Interface>,          
    _gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
) {
    let ui = ui_op.unwrap();

    let dark_mode = ui.visuals().dark_mode;
    let faded_color = match dark_mode {
        true => egui::Color32::from_rgba_premultiplied(25, 25, 25, 125),
        false => egui::Color32::from_rgba_premultiplied(145, 145, 145, 50),
    };

    StripBuilder::new(ui)
        .size(Size::remainder())
        .size(Size::relative(0.6))
        .size(Size::remainder())
        .size(Size::exact(40.0))
        .size(Size::exact(28.0))
        .vertical(|mut strip| {
            strip.empty();
            strip.strip(|builder| {

                builder
                    .size(Size::exact(50.0))
                    .size(Size::exact(80.0))
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                0.0,
                                egui::Color32::TRANSPARENT,
                            );
                        });
                        strip.cell(|ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                5.0,
                                faded_color,
                            );
                        })
                    });
            });
            strip.empty();
            strip.strip(|builder| {
                builder
                    .size(Size::exact(50.0))
                    .size(Size::exact(200.0))
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        strip.empty();
                        strip.cell(|ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                5.0,
                                faded_color,
                            );
                        });
                    });
            });
        });

        /*
        gui_interface.register_boundary(
            gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
        */
    



}