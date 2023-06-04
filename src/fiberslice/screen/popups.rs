/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy_egui::egui;
use egui::Context;

pub(crate) struct PopupsView {

}

impl PopupsView {
    pub fn init() -> PopupsView {
        PopupsView {
        }
    }

    pub fn popups_ui(&mut self, 
        ctx: &Context, 
    ) {

        egui::Window::new("Test")
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.label("Label test");
            let button = ui.button("button test");
        });
    }
}