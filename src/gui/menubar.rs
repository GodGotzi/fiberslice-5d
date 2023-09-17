use three_d::egui::{self, Ui};

use crate::config;
use crate::gui;
use crate::import::import_gcode;
use crate::import::import_model;

use super::GuiContext;

pub struct Menubar;

impl Menubar {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Menubar> for Menubar {
    fn show(&mut self, ctx: &egui::Context, gui_context: &mut GuiContext) {
        let boundary = egui::TopBottomPanel::top("menubar")
            .default_height(config::gui::MENUBAR_H)
            .show(ctx, |ui: &mut Ui| {
                egui::menu::bar(ui, |ui| {
                    file_button(ui, gui_context);
                    edit_button(ui, gui_context);
                    view_button(ui, gui_context);
                    settings_button(ui, gui_context);
                    help_button(ui, gui_context);
                });
            })
            .response
            .into();

        gui_context.application.context.boundaries_mut().menubar = boundary;
    }
}

fn file_button(ui: &mut Ui, gui_context: &mut GuiContext) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        let manipulator = gui_context.manipulator.clone();
        let context = gui_context.context.clone();

        build_sub_menu(ui, "Import", || {
            import_model(context.clone(), manipulator.clone())
        });
        build_sub_menu(ui, "Import GCode", || import_gcode(context, manipulator));
    });
}

fn edit_button(ui: &mut Ui, _gui_context: &mut GuiContext) {
    ui.menu_button("Edit", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn view_button(ui: &mut Ui, _gui_context: &mut GuiContext) {
    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn settings_button(ui: &mut Ui, _gui_context: &mut GuiContext) {
    ui.menu_button("Settings", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn help_button(ui: &mut Ui, _gui_context: &mut GuiContext) {
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
