/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

mod side;
mod addons;
mod taskbar;
mod menubar;

use std::collections::HashMap;

use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Color32};
use crate::view::ViewInterface;

use super::gui;
use super::{gui::{GuiInterface, GuiComponent}, utils::Creation, EventWrapper};

pub struct Screen {
    side: side::SideView,
    addons: addons::ViewAddons,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
}

impl Creation for Screen {
    fn create() -> Screen {
        Screen {
            side: side::SideView::create(),
            addons: addons::ViewAddons::create(),
            menubar: menubar::Menubar::create(),
            taskbar: taskbar::Taskbar::create(),
        }
    }
}

impl GuiComponent<Screen> for Screen {

    fn show(&mut self, ctx: &egui::Context,
        _ui: Option<&mut egui::Ui>,
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>
    ) {
        self.side.show(ctx, None, view_interface, gui_interface, gui_events);
        self.menubar.show(ctx, None,  view_interface, gui_interface, gui_events);
        self.taskbar.show(ctx, None, view_interface, gui_interface, gui_events);

        let frame = egui::containers::Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame)
        .show(ctx, |ui| {
            self.addons.show(ctx, Some(ui), view_interface, gui_interface, gui_events);
        });
    }
    
}