/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::prelude::*;

use crate::view::{ViewInterface};

use self::{screen::Screen, gui::GuiInterface};
use self::gui::GuiResizeEvent;

pub mod utils;
pub mod screen;
pub mod gui;
mod options;

#[derive(Resource)]
pub struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {

    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }

    pub fn ui_frame(&mut self, ctx: &bevy_egui::egui::Context, 
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,       
        events_resize: &mut EventWriter<GuiResizeEvent>
    ) {
        self.screen.ui(ctx, view_interface, gui_interface, events_resize);
    }

}