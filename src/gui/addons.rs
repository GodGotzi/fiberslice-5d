/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::prelude::{Vec2, ResMut};
use egui::{Ui, Response, Color32};
use egui_extras::{StripBuilder, Size};

use crate::{gui, prelude::{AsyncWrapper, Mode}, config};

use super::{Boundary, screen::Screen};

mod prepare;
mod force_analytics;
mod preview;
mod monitor;

type AddonStripBuilderClosure = dyn Fn(StripBuilder, &mut ResMut<gui::Interface>, &mut ResMut<AsyncWrapper>, Color32);

pub fn create_addon_strip_builder(
    ui: &mut Ui,
    boundary: Boundary,
    gui_interface: &mut ResMut<gui::Interface>,          
    item_wrapper: &mut ResMut<AsyncWrapper>,
    shaded_color: Color32,
    build: Box<AddonStripBuilderClosure>) -> Response {

    StripBuilder::new(ui)
        .size(Size::exact(boundary.location.x + 5.0))
        .size(Size::exact(boundary.size.x - 10.0))
        .size(Size::remainder())
        .horizontal(|mut strip| {
            strip.empty();
            strip.strip(|builder| {
                builder
                    .size(Size::exact(boundary.location.y + 5.0))
                    .size(Size::exact(boundary.size.y - 10.0))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.empty();
                        strip.strip(|builder| {
                            build(builder, gui_interface, item_wrapper, shaded_color);
                        });
                        strip.empty();
                    });
            });
            strip.empty();
        })
}


pub mod orientation {
    use bevy::prelude::ResMut;
    use bevy_egui::egui::{Ui, self, Direction, Button, ImageButton};
    use egui_extras::Size;
    use egui_grid::GridBuilder;

    use crate::{prelude::AsyncWrapper, gui::icon};

    pub fn show(ui: &mut Ui, _item_wrapper: &mut ResMut<AsyncWrapper>) {
        
        let layout = egui::Layout {
            main_dir: Direction::RightToLeft,
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
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::initial(30.0))
            .cell(Size::remainder())
            .show(ui, |mut grid| {
                grid.empty();
                grid.cell(|ui| {
                    let icon = icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Default);

                    let image_button = ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Normal");
                    }
                });

                grid.cell(|ui| {
                    
                    ui.add_sized([30., 30.], Button::new(""));
                });

                grid.cell(|ui| {
                    let icon = icon::ICONTABLE.get_orientation_icon(crate::view::Orientation::Default);

                    let image_button = ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);

                    let response = ui.add_sized([30., 30.], image_button);

                    if response.clicked() {
                        println!("Clicked Front");
                    }
                });

                grid.cell(|ui| {
                    ui.add_sized([30., 30.], Button::new(""));
                });

                grid.cell(|ui| {
                    ui.add_sized([30., 30.], Button::new(""));
                });
                grid.empty();
            });
        

    }


}

pub struct Addons {

}

impl Addons {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Addons> for Addons {

    fn show(&mut self, ctx: &egui::Context,
        ui_op: Option<&mut Ui>,
        mode_ctx: Option<&mut Mode>,
        gui_interface: &mut bevy::prelude::ResMut<gui::Interface>,          
        item_wrapper: &mut ResMut<AsyncWrapper>
    ) {
        let ui = ui_op.unwrap();

        let window_size = ui.available_size();

        let settingsbar_width = Screen::get_settingsbar_width(item_wrapper);

        let boundary = Boundary {
            location: Vec2::new(config::gui::TOOLBAR_W + 8.0, -3.0),
            size: Vec2::new(
                window_size.x - config::gui::TOOLBAR_W - 32.0 - settingsbar_width, 
                window_size.y - config::gui::MODEBAR_H - 5.0),
        };

        match mode_ctx.unwrap() {
            Mode::Prepare => prepare::show(ctx, ui, boundary, gui_interface, item_wrapper),
            Mode::Preview => preview::show(ctx, ui, boundary, gui_interface, item_wrapper),
            Mode::Monitor => monitor::show(ctx, ui, boundary, gui_interface, item_wrapper),
            Mode::ForceAnalytics => force_analytics::show(ctx, ui, boundary, gui_interface, item_wrapper),
        }
    }

}