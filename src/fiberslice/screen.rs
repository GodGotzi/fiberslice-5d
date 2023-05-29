mod side;
mod view;

use std::sync::Arc;
use eframe::CreationContext;
use egui::{Context, Modifiers, Ui};
use egui::WidgetType::Slider;
use crate::fiberslice::screen::menu::menubar_ui;

mod menu {
    use egui::{Context, Ui};

    pub fn menubar_ui(ctx: &Context, screen: &mut super::Screen) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                theme_button(ui, screen);
                ui.separator();
                file_button(ui, screen);
                edit_button(ui, screen);
                view_button(ui, screen);
                settings_button(ui, screen);
                help_button(ui, screen);
            });
        });
    }

    fn file_button(ui: &mut Ui, screen: &mut super::Screen) {
        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn edit_button(ui: &mut Ui, screen: &mut super::Screen) {
        ui.menu_button("Edit", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn view_button(ui: &mut Ui, screen: &mut super::Screen) {
        ui.menu_button("View", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn settings_button(ui: &mut Ui, screen: &mut super::Screen) {
        ui.menu_button("Settings", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn help_button(ui: &mut Ui, screen: &mut super::Screen) {
        ui.menu_button("Help", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn theme_button(ui: &mut Ui, screen: &mut super::Screen) {
        let clicked = match screen.toggle_theme {
            true => ui.button("💡").clicked().clone(),
            false => ui.button("🌙").clicked().clone(),
        };

        if screen.toggle_theme {
            handle_toggle_theme(ui,clicked, screen);
        } else {
            handle_toggle_theme(ui, clicked, screen);
        }
    }

    fn handle_toggle_theme(ui: &mut Ui, toggle: bool, screen: &mut super::Screen) {
        if toggle {
            screen.toggle_theme = !screen.toggle_theme;

            if screen.toggle_theme {
                ui.ctx().set_visuals(egui::Visuals::dark());
            } else {
                ui.ctx().set_visuals(egui::Visuals::light());
            }
        }
    }
}

pub struct Screen {
    toggle_theme: bool,
    side_view_data: side::SideView,
    view_data: view::View,
}

impl Screen {
    pub fn new(cc: &CreationContext) -> Screen {

        let screen = Screen {
            toggle_theme: true,
            side_view_data: side::SideView::init(),
            view_data: view::View::init(cc),
        };

        screen
    }

    pub(crate) fn ui(&mut self, ctx: &Context) {
        menubar_ui(ctx, self);
        self.side_view_data.side_panel_ui(ctx);
        self.view_data.view_panel_ui(ctx);
    }
}