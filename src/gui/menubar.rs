use three_d::egui::{self, Ui};

use crate::application::ApplicationContext;
use crate::config;
use crate::gui;

pub struct Menubar;

impl Menubar {
    pub fn new() -> Self {
        Self {}
    }
}

impl gui::Component<Menubar> for Menubar {
    fn show(&mut self, ctx: &egui::Context, app: &mut ApplicationContext) {
        let boundary = egui::TopBottomPanel::top("menubar")
            .default_height(config::gui::MENUBAR_H)
            .show(ctx, |ui: &mut Ui| {
                egui::menu::bar(ui, |ui| {
                    file_button(ui, app);
                    edit_button(ui, app);
                    view_button(ui, app);
                    settings_button(ui, app);
                    help_button(ui, app);
                });
            })
            .response
            .into();

        app.boundaries_mut().menubar = boundary;
    }
}

fn file_button(ui: &mut Ui, _app: &mut ApplicationContext) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn edit_button(ui: &mut Ui, _app: &mut ApplicationContext) {
    ui.menu_button("Edit", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn view_button(ui: &mut Ui, _app: &mut ApplicationContext) {
    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn settings_button(ui: &mut Ui, _app: &mut ApplicationContext) {
    ui.menu_button("Settings", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn help_button(ui: &mut Ui, _app: &mut ApplicationContext) {
    ui.menu_button("Help", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}
