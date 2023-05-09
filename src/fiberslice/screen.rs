mod side;
mod view;

use egui::{Context, Modifiers, Ui};
use egui::WidgetType::Slider;
use crate::fiberslice::screen::menu::menubar_ui;

mod menu {
    use egui::{Context, Ui};

    pub fn menubar_ui(ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_button(ui);
                edit_button(ui);
                window_button(ui);
                view_button(ui);
                settings_button(ui);
                help_button(ui);
            });
        });
    }

    fn file_button(ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn edit_button(ui: &mut Ui) {
        ui.menu_button("Edit", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn window_button(ui: &mut Ui) {
        ui.menu_button("Window", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn view_button(ui: &mut Ui) {
        ui.menu_button("View", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn settings_button(ui: &mut Ui) {
        ui.menu_button("Settings", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn help_button(ui: &mut Ui) {
        ui.menu_button("Help", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }
}

pub struct Screen {
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
        }
    }

    pub(crate) fn ui(&mut self, ctx: &Context) {
        menubar_ui(ctx);
        side::side_panel_ui(ctx);
        view::view_panel_ui(ctx);
    }
}