use bevy_egui::egui;
use egui::Ui;

use crate::actions::file::load_gcode;
use crate::config;
use crate::ui::{Component, UiData};

pub struct Menubar;

impl Menubar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Menubar {
    fn show(&mut self, ctx: &egui::Context, data: UiData) {
        let boundary = egui::TopBottomPanel::top("menubar")
            .default_height(config::gui::MENUBAR_H)
            .show(ctx, |ui: &mut Ui| {
                egui::menu::bar(ui, |ui| {
                    file_button(ui, data);
                    edit_button(ui, data);
                    window_button(ui, data);
                    view_button(ui, data);
                    //settings_button(ui, data);
                    help_button(ui, data);
                });
            })
            .response
            .into();

        data.raw.borrow_mut().holder.menubar.set_boundary(boundary);
    }
}

fn file_button(ui: &mut Ui, data: UiData) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        //let manipulator = gui_context.manipulator.clone();
        //let context = gui_context.context.clone();

        build_sub_menu(ui, "Load GCode", || load_gcode(data));
    });
}

fn edit_button(ui: &mut Ui, _data: UiData) {
    ui.menu_button("Edit", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn window_button(ui: &mut Ui, data: UiData) {
    ui.menu_button("Window", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        ui.checkbox(&mut data.raw.borrow_mut().holder.addons.enabled, "Addons");
        ui.separator();

        ui.checkbox(&mut data.raw.borrow_mut().holder.modebar.enabled, "ModeBar");
        ui.checkbox(&mut data.raw.borrow_mut().holder.toolbar.enabled, "ToolBar");
        ui.checkbox(&mut data.raw.borrow_mut().holder.taskbar.enabled, "TaskBar");
        ui.checkbox(
            &mut data.raw.borrow_mut().holder.settingsbar.enabled,
            "Settings",
        );
    });
}

fn view_button(ui: &mut Ui, _data: UiData) {
    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn help_button(ui: &mut Ui, _data: UiData) {
    ui.menu_button("Help", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn build_sub_menu(ui: &mut Ui, title: &str, action: impl FnOnce()) {
    if ui.button(title).clicked() {
        action();
    }
}
