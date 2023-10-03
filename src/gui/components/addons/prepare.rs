use bevy_egui::egui;
use egui_extras::Size;

use crate::{
    config::gui::shaded_color,
    gui::{Boundary, UiData},
};

pub fn show(_ctx: &egui::Context, ui: &mut egui::Ui, data: UiData, boundary: Boundary) {
    let shaded_color = shaded_color(ui.visuals().dark_mode);

    let _response = super::create_addon_strip_builder(
        ui,
        data,
        boundary,
        shaded_color,
        Box::new(|builder, data, shaded_color| {
            builder
                .size(Size::remainder())
                .size(Size::relative(0.6))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(Size::exact(40.0))
                            .size(Size::remainder())
                            .vertical(|mut strip| {
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::remainder())
                                        .size(Size::exact(220.0))
                                        .horizontal(|mut strip| {
                                            strip.empty();
                                            strip.cell(|ui| {
                                                ui.painter().rect_filled(
                                                    ui.available_rect_before_wrap(),
                                                    0.0,
                                                    shaded_color,
                                                );

                                                super::orientation::show(ui, data);
                                            });
                                        });
                                });
                                strip.empty();
                            });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::exact(80.0))
                            .size(Size::remainder())
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    ui.painter().rect_filled(
                                        ui.available_rect_before_wrap(),
                                        0.0,
                                        shaded_color,
                                    );
                                });

                                strip.empty();
                            });
                    });
                    strip.empty();
                });
        }),
    );

    /*
    gui_interface.register_boundary(
        gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
    );
    */
}
