use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use egui::Context;

use crate::fiberslice::utils::Creation;
use crate::view::ViewInterface;
use crate::fiberslice::gui::*;

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
        _view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        _events_resize: &mut EventWriter<GuiResizeEvent>
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