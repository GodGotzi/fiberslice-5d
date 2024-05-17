use egui::{Layout, Ui};
use egui_extras::Size;

use crate::{
    config::gui::shaded_color,
    ui::{boundary::Boundary, UiData},
};

pub fn show(_ctx: &egui::Context, ui: &mut Ui, data: &mut UiData, boundary: Boundary) {
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
                .size(Size::exact(25.0))
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

                                                super::orientation::show(ui, data);
                                            });
                                        });
                                });
                                strip.empty();
                            });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::remainder())
                            .size(Size::exact(25.0))
                            .horizontal(|mut strip| {
                                strip.empty();
                                strip.cell(|ui| {
                                    ui.with_layout(
                                        Layout::top_down_justified(egui::Align::Center),
                                        |ui| {
                                            let mut num = 0;

                                            ui.spacing_mut().slider_width = ui.available_height();

                                            let slider = egui::Slider::new(&mut num, 0..=120)
                                                .orientation(egui::SliderOrientation::Vertical);
                                            ui.add_sized(ui.available_size(), slider);
                                        },
                                    );
                                });
                            });
                    });
                    strip.empty();
                    strip.strip(|builder| {
                        builder
                            .size(Size::remainder())
                            .size(Size::relative(0.6))
                            .size(Size::remainder())
                            .horizontal(|mut strip| {
                                strip.empty();
                                strip.cell(|ui| {
                                    let mut num = 0;

                                    ui.spacing_mut().slider_width = ui.available_width();

                                    let slider = egui::Slider::new(&mut num, 0..=120)
                                        .orientation(egui::SliderOrientation::Horizontal);
                                    ui.add(slider);
                                });
                                strip.empty();
                            });
                    });
                });
        }),
    );

    /*
    gui_interface.register_boundary(
        gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
    );
    */
}
