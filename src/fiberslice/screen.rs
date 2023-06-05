/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

mod side;
mod popups;
mod taskbar;
mod menubar;

use bevy::prelude::{ResMut, EventWriter};
use bevy_egui::egui::{self, Color32};
use crate::view::ViewInterface;

use super::{gui::{GuiInterface, GuiResizeEvent, GuiComponent}, utils::Creation};

pub struct Screen {
    side: side::SideView,
    popups: popups::PopupsView,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            side: side::SideView::create(),
            popups: popups::PopupsView::create(),
            menubar: menubar::Menubar::create(),
            taskbar: taskbar::Taskbar::create(),
        }
    }

    pub(crate) fn ui(&mut self, 
        ctx: &egui::Context, 
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        events_resize: &mut EventWriter<GuiResizeEvent>
    ) {

        self.side.show(ctx, view_interface, gui_interface, events_resize);
        self.menubar.show(ctx,  view_interface, gui_interface, events_resize);
        self.taskbar.show(ctx, view_interface, gui_interface, events_resize);

        let frame = egui::containers::Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame)
        .show(ctx, |_ui| {
            self.popups.show(ctx, view_interface, gui_interface, events_resize);


        });

    }
}