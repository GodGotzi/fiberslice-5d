mod addons;
mod menubar;
mod settingsbar;
mod taskbar;
mod modebar;
mod toolbar;

pub mod screen;

use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui::Ui;

use crate::prelude::*;

use crate::gui;

pub trait Component<T> {
    fn show(&mut self, ctx: &egui::Context,
        ui: Option<&mut Ui>,
        mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<gui::Interface>,          
        gui_events: &mut HashMap<ItemType, AsyncPacket<Item>>
    );
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

pub enum Theme {
    Light,
    Dark
}

#[derive(Resource)]
pub struct Interface {
    touch: bool,
    theme: Theme,
    boundaries: Vec<Boundary>
}

impl Interface {

    pub fn new() -> Self {
        Self {
            touch: false,
            theme: Theme::Light,
            boundaries: Vec::new()
        }
    }

    pub fn is_touch(&self) -> bool {
        self.touch
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

    pub fn check_boundaries(&mut self, cursor_vec: Vec2) {

        for boundary in self.boundaries.iter() {
            if Self::check_boundary(boundary, 0., cursor_vec) {
                self.touch = true;

                self.boundaries.clear();
                return;
            } else {
                self.touch = false;
            }
        }
    
        self.boundaries.clear();
    
    }

    fn check_boundary(boundary: &Boundary, additional_broder: f32, cursor_vec: Vec2) -> bool {

        if boundary.location.x - additional_broder <= cursor_vec.x && boundary.location.x + boundary.size.x + additional_broder >= cursor_vec.x 
            && boundary.location.y - additional_broder <= cursor_vec.y && boundary.location.y + boundary.size.y + additional_broder >= cursor_vec.y
            {
            
            return true;
        }
    
        false
    }

}

pub fn check_touch(
    mut gui_interface: ResMut<self::Interface>,
    mut contexts: EguiContexts,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let ctx = contexts.ctx_mut();

    if ctx.is_using_pointer() {
        gui_interface.touch = true;
        return;
    }

    let opt_cursor = ctx.pointer_hover_pos();

    if opt_cursor.is_none() {
        return;
    }

    let cursor_pos = opt_cursor.unwrap();

    if !mouse_buttons.pressed(MouseButton::Left) && !mouse_buttons.pressed(MouseButton::Middle) {
        gui_interface.touch = false;
        return;
    }

    gui_interface.check_boundaries(Vec2::new(cursor_pos.x, cursor_pos.y));

}