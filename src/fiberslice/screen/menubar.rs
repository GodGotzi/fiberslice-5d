use std::collections::HashMap;

use bevy::prelude::ResMut;
use bevy_egui::egui;
use egui::Ui;

use crate::fiberslice::{gui::{GuiInterface, Boundary, GuiComponent, self}, utils::Creation, EventWrapper};

pub struct Menubar;


impl Creation for Menubar {
    fn create() -> Self {
        Self {}
    }
}


impl GuiComponent<Menubar> for Menubar {

    fn show(&mut self, ctx: &egui::Context,
        _ui: Option<&mut Ui>,
        _view_interface: &mut ResMut<crate::view::ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        _gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>
    ) {
        let response = egui::TopBottomPanel::top("menubar").show(ctx, |ui: &mut Ui| {
            egui::menu::bar(ui, |ui| {
                theme_button(ui, gui_interface);
                ui.separator();
                
                file_button(ui, gui_interface);
                edit_button(ui, gui_interface);
                view_button(ui, gui_interface);
                settings_button(ui, gui_interface);
                help_button(ui, gui_interface);
            });
        }).response;

        let rect = response.rect;

        gui_interface.menubar_boundary = Some(
            Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }

}

fn file_button(ui: &mut Ui, _gui_interface: &mut GuiInterface) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn edit_button(ui: &mut Ui, _gui_interface: &mut GuiInterface) {
    ui.menu_button("Edit", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn view_button(ui: &mut Ui, _gui_interface: &mut GuiInterface) {
    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn settings_button(ui: &mut Ui, _gui_interface: &mut GuiInterface) {
    ui.menu_button("Settings", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn help_button(ui: &mut Ui, _gui_interface: &mut GuiInterface) {
    ui.menu_button("Help", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn theme_button(ui: &mut Ui, gui_interface: &mut GuiInterface) {
    let clicked = match gui_interface.toggle_theme {
        true => ui.button("💡").clicked(),
        false => ui.button("🌙").clicked(),
    };

    handle_toggle_theme(ui, clicked, gui_interface);
}

fn handle_toggle_theme(ui: &mut Ui, toggle: bool, gui_interface: &mut GuiInterface) {
    if toggle {
        gui_interface.toggle_theme = !gui_interface.toggle_theme;

        if gui_interface.toggle_theme {
            ui.ctx().set_visuals(egui::Visuals::dark());
        } else {
            ui.ctx().set_visuals(egui::Visuals::light());
        }
    }
}