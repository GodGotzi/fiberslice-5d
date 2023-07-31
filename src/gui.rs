mod addons;
pub mod icon;
mod menubar;
mod modebar;
mod settingsbar;
mod taskbar;
mod toolbar;

pub mod screen;

use three_d::egui;

use crate::prelude::*;

use crate::gui;

pub trait Component<T> {
    fn show(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        mode_ctx: Option<&mut Mode>,
        gui_interface: &mut gui::Interface,
        item_wrapper: &mut AsyncWrapper,
    );
}

pub struct Boundary {
    pub location: egui::Vec2,
    pub size: egui::Vec2,
}

impl Boundary {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            location: egui::Vec2::new(x, y),
            size: egui::Vec2::new(width, height),
        }
    }
}

pub enum Theme {
    Light,
    Dark,
}

pub struct Interface {
    theme: Theme,
    boundaries: Vec<Boundary>,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            theme: Theme::Light,
            boundaries: Vec::new(),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn register_boundary(&mut self, boundary: Boundary) {
        self.boundaries.push(boundary);
    }
}
