
use bevy::prelude::ResMut;
use egui::Ui;
use egui_extras::Size;

use crate::{prelude::*, gui::{self, Boundary}, config::gui::shaded_color};

pub fn show(
    _ctx: &egui::Context,
    ui: &mut Ui,
    boundary: Boundary,
    gui_interface: &mut bevy::prelude::ResMut<gui::Interface>,          
    item_wrapper: &mut ResMut<AsyncWrapper>,
) {

    let shaded_color = shaded_color(ui.visuals().dark_mode);

    let _response = super::create_addon_strip_builder( ui, boundary, gui_interface, item_wrapper, shaded_color,
        Box::new(|builder, _gui_interface, item_wrapper, shaded_color| {

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
                                .size(Size::exact(200.0))
                                .horizontal(|mut strip| {
                                    strip.empty();
                                    strip.cell(|ui| {
                                        ui.painter().rect_filled(
                                            ui.available_rect_before_wrap(),
                                            5.0,
                                            shaded_color,
                                        );

                                        super::orientation::show(ui, item_wrapper);
                                    });
                                });
                            });
                            strip.empty();

                        });
                });
                strip.empty();
                strip.empty();
            });
    }));
        

        /*
        gui_interface.register_boundary(
            gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
        */
    



}