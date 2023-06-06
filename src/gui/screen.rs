/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/


use std::collections::HashMap;

use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Color32};
use crate::prelude::AsyncPacket;

use super::{gui, side, addons, menubar, taskbar};
use crate::utils::Creation;

pub struct Screen {
    side: side::SideView,
    addons: addons::Addons,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
}

impl Creation for Screen {
    fn create() -> Screen {
        Screen {
            side: side::SideView::create(),
            addons: addons::Addons::create(),
            menubar: menubar::Menubar::create(),
            taskbar: taskbar::Taskbar::create(),
        }
    }
}

impl gui::Component<Screen> for Screen {

    fn show(&mut self, ctx: &egui::Context,
        _ui: Option<&mut egui::Ui>,
        gui_interface: &mut ResMut<gui::Interface>,          
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
    ) {
        self.side.show(ctx, None, gui_interface, gui_events);
        self.menubar.show(ctx, None, gui_interface, gui_events);
        self.taskbar.show(ctx, None, gui_interface, gui_events);

        let frame = egui::containers::Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame)
        .show(ctx, |ui| {
            self.addons.show(ctx, Some(ui), gui_interface, gui_events);
        });
    }
    
}