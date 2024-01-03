use egui_extras::Size;
use three_d::egui;

use crate::{
    config::gui::shaded_color,
    ui::{boundary::Boundary, UiData},
};

pub fn show(_ctx: &egui::Context, ui: &mut egui::Ui, data: &mut UiData, boundary: Boundary) {
    let shaded_color = shaded_color(ui.visuals().dark_mode);

    let _response = super::create_addon_strip_builder(
        ui,
        data,
        boundary,
        shaded_color,
        Box::new(|builder, app, shaded_color| {
            builder
                .size(Size::remainder())
                .size(Size::relative(0.6))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(Size::exact(50.0))
                            .size(Size::remainder())
                            .vertical(|mut strip| {
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::remainder())
                                        .size(Size::exact(240.0))
                                        .horizontal(|mut strip| {
                                            strip.empty();
                                            strip.cell(|ui| {
                                                ui.painter().rect_filled(
                                                    ui.available_rect_before_wrap(),
                                                    0.0,
                                                    shaded_color,
                                                );

                                                super::orientation::show(ui, app);
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
                    strip.strip(|builder| {
                        builder
                            .size(Size::remainder())
                            .size(Size::exact(60.0))
                            .vertical(|mut strip| {
                                strip.empty();
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::remainder())
                                        .size(Size::relative(0.4))
                                        .size(Size::remainder())
                                        .horizontal(|mut strip| {
                                            strip.empty();
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
                            });
                    });
                });
        }),
    );
}
