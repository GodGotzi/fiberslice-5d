mod side;
mod popups;

use bevy::prelude::{ResMut, EventWriter, Vec2};
use bevy_egui::egui;
use crate::{fiberslice::screen::menu::menubar_ui, view::ViewInterface};

use super::gui::GuiInterface;

pub enum GuiResizeEvent {
    Side(f32)
}

pub struct Boundary {
    pub location: Vec2,
    pub size: Vec2
}

impl Boundary {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            location: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }
}

mod menu {
    use bevy::prelude::ResMut;
    use bevy_egui::egui;
    use egui::Ui;

    use crate::fiberslice::gui::GuiInterface;

    use super::Boundary;

    pub fn menubar_ui(ctx: &egui::Context, screen: &mut super::Screen, gui_interface: &mut ResMut<GuiInterface>) {
        let response = egui::TopBottomPanel::top("menu_bar").show(ctx, |ui: &mut Ui| {
            egui::menu::bar(ui, |ui| {
                theme_button(ui, screen);
                ui.separator();
                file_button(ui, screen);
                edit_button(ui, screen);
                view_button(ui, screen);
                settings_button(ui, screen);
                help_button(ui, screen);
            });
        }).response;

        let rect = response.rect;

        gui_interface.menu_boundary = Some(
            Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
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
    side_view: side::SideView,
    popups_view: popups::PopupsView,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            toggle_theme: true,
            side_view: side::SideView::init(),
            popups_view: popups::PopupsView::init(),
        }
    }

    pub(crate) fn ui(&mut self, 
        ctx: &egui::Context, 
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        events_resize: &mut EventWriter<GuiResizeEvent>
    ) {
        menubar_ui(ctx, self, gui_interface);
        self.side_view.side_panel_ui(ctx, view_interface, gui_interface, events_resize);
        self.popups_view.popups_ui(ctx);

    }
}