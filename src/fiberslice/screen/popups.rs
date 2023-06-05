/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy_egui::egui;

use crate::fiberslice::{gui::GuiComponent, utils::Creation};

pub struct PopupsView;

impl Creation for PopupsView {
    fn create() -> Self {
        Self {}
    }
}

impl GuiComponent<PopupsView> for PopupsView {

    fn show(&mut self, ctx: &egui::Context, 
        _view_interface: &mut bevy::prelude::ResMut<crate::view::ViewInterface>,
        _gui_interface: &mut bevy::prelude::ResMut<crate::fiberslice::gui::GuiInterface>,          
        _events_resize: &mut bevy::prelude::EventWriter<crate::fiberslice::gui::GuiResizeEvent>
    ) {
        egui::Window::new("Test")
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.label("Label test");
            let _button = ui.button("button test");
        });
    }

}