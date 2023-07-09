use bevy::prelude::ResMut;
use bevy_egui::egui;
use egui::Ui;

use crate::{prelude::*, config};
use crate::gui;

pub struct Menubar;


impl Menubar {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Menubar> for Menubar {

    fn show(&mut self, ctx: &egui::Context,
        _ui: Option<&mut Ui>,
        _mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<gui::Interface>,          
        _item_wrapper: &mut ResMut<AsyncWrapper>,
    ) {
        let response = egui::TopBottomPanel::top("menubar")
            .default_height(config::gui::MENUBAR_H)
            .show(ctx, |ui: &mut Ui| {
                egui::menu::bar(ui, |ui| {
                    file_button(ui, gui_interface);
                    edit_button(ui, gui_interface);
                    view_button(ui, gui_interface);
                    settings_button(ui, gui_interface);
                    help_button(ui, gui_interface);
            });
        }).response;

        let rect = response.rect;

        gui_interface.register_boundary(
            gui::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }

}

fn file_button(ui: &mut Ui, _gui_interface: &mut gui::Interface) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn edit_button(ui: &mut Ui, _gui_interface: &mut gui::Interface) {
    ui.menu_button("Edit", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn view_button(ui: &mut Ui, _gui_interface: &mut gui::Interface) {
    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn settings_button(ui: &mut Ui, _gui_interface: &mut gui::Interface) {
    ui.menu_button("Settings", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn help_button(ui: &mut Ui, _gui_interface: &mut gui::Interface) {
    ui.menu_button("Help", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}