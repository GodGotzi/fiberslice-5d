use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use egui::Context;

use crate::{prelude::*, config};
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
        _mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<gui::Interface>,          
        _item_wrapper: &mut ResMut<AsyncWrapper>, 
    ) {
        let response = egui::TopBottomPanel::bottom("taskbar")
            .default_height(config::gui::TASKBAR_H)
            .show(ctx, |ui: &mut Ui| {
                egui::menu::bar(ui, |ui| {

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        theme_button(ui, gui_interface);
                    });

                });
            }).response;

        let rect = response.rect;

        gui_interface.register_boundary(
            gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }
}

fn theme_button(ui: &mut Ui, gui_interface: &mut gui::Interface) {
    let clicked = match gui_interface.theme() {
        gui::Theme::Dark => ui.button("💡").clicked(),
        gui::Theme::Light=> ui.button("🌙").clicked(),
    };

    if clicked {
        gui_interface.toggle_theme();
    }
}