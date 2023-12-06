use egui::Ui;
use egui_extras::Size;
use three_d::egui;

use crate::{
    config::gui::{addons, shaded_color},
    ui::{boundary::Boundary, UiData},
};

pub fn show(
    addons: super::Addons,
    _ctx: &egui::Context,
    ui: &mut Ui,
    data: &mut UiData,
    boundary: Boundary,
) {
    let shaded_color = shaded_color(ui.visuals().dark_mode);

    let _response = super::create_addon_strip_builder(
        addons,
        ui,
        data,
        boundary,
        shaded_color,
        &|addons, builder, data, shaded_color| {
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

                                                super::orientation::show(addons, ui, data);
                                            });
                                        });
                                });
                                strip.empty();
                            });
                    });
                    strip.empty();
                    strip.empty();
                });
        },
    );
}
