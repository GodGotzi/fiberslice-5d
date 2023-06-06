use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use egui::Context;

use crate::fiberslice::gui::Boundary;
use crate::fiberslice::gui::GuiComponent;
use crate::fiberslice::gui::GuiInterface;
use crate::fiberslice::utils::Creation;
use crate::view::ViewInterface;
use crate::fiberslice::gui;
use crate::fiberslice::EventWrapper;

pub struct Taskbar {

}

impl Creation for Taskbar {
    fn create() -> Self {
        Self {
        }
    }
}

impl GuiComponent<Taskbar> for Taskbar {

    fn show(&mut self, ctx: &Context,
        _ui: Option<&mut Ui>,
        _view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        _gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>
    ) {
        let response = egui::TopBottomPanel::bottom("taskbar").show(ctx, |ui: &mut Ui| {
            egui::menu::bar(ui, |_ui| {
                
            });
        }).response;

        let rect = response.rect;

        gui_interface.taskbar_boundary = Some(
            Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }

}