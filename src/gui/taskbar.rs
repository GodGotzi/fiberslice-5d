use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use egui::Context;

use crate::prelude::*;
use crate::{gui, utils::Creation};

pub struct Taskbar {

}

impl Creation for Taskbar {
    fn create() -> Self {
        Self {
        }
    }
}

impl gui::Component<Taskbar> for Taskbar {

    fn show(&mut self, ctx: &Context,
        _ui: Option<&mut Ui>,
        gui_interface: &mut ResMut<gui::Interface>,          
        _gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
    ) {
        let response = egui::TopBottomPanel::bottom("taskbar").show(ctx, |ui: &mut Ui| {
            egui::menu::bar(ui, |_ui| {
                
            });
        }).response;

        let rect = response.rect;

        gui_interface.taskbar_boundary = Some(
            gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }

}