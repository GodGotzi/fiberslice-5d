mod side;

use bevy_egui::egui;
use crate::fiberslice::screen::menu::menubar_ui;

mod menu {
    use bevy_egui::egui;
    use egui::Ui;

    pub fn menubar_ui(ctx: &egui::Context, screen: &mut super::Screen) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui: &mut Ui| {
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

    fn file_button(ui: &mut Ui, _screen: &mut super::Screen) {
        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn edit_button(ui: &mut Ui, _screen: &mut super::Screen) {
        ui.menu_button("Edit", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn view_button(ui: &mut Ui, _screen: &mut super::Screen) {
        ui.menu_button("View", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn settings_button(ui: &mut Ui, _screen: &mut super::Screen) {
        ui.menu_button("Settings", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn help_button(ui: &mut Ui, _screen: &mut super::Screen) {
        ui.menu_button("Help", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
        });
    }

    fn theme_button(ui: &mut Ui, screen: &mut super::Screen) {
        let clicked = match screen.toggle_theme {
            true => ui.button("💡").clicked(),
            false => ui.button("🌙").clicked(),
        };

        handle_toggle_theme(ui, clicked, screen);
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
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            toggle_theme: true,
            side_view_data: side::SideView::init(),
        }
    }

    pub(crate) fn ui(&mut self, ctx: &egui::Context) {
        menubar_ui(ctx, self);
        self.side_view_data.side_panel_ui(ctx);
    }
}