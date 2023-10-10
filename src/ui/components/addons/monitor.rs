use egui::Ui;
use egui_extras::Size;
use three_d::egui;

use crate::{application::ApplicationContext, config::gui::shaded_color, gui::Boundary};

pub fn show(_ctx: &egui::Context, ui: &mut Ui, app: &mut ApplicationContext, boundary: Boundary) {
    let shaded_color = shaded_color(ui.visuals().dark_mode);

    let _response = super::create_addon_strip_builder(
        ui,
        app,
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
                                                    5.0,
                                                    shaded_color,
                                                );

                                                super::orientation::show(ui, app);
                                            });
                                        });
                                });
                                strip.empty();
                            });
                    });
                    strip.empty();
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
