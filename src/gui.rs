/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

mod components;
mod icon;
pub mod menubar;
pub mod modebar;
pub mod settingsbar;
pub mod taskbar;
pub mod toolbar;

use three_d::egui::{self, Response};

use crate::{application::ApplicationContext, prelude::*, view::environment::Environment};

use self::components::addons;

pub struct GuiContext<'a> {
    pub application_ctx: &'a mut ApplicationContext,
    pub environment: &'a mut Environment,
}

pub trait Component<T> {
    fn show(&mut self, ctx: &egui::Context, gui_context: &mut GuiContext);
}

pub trait InnerComponent<T> {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, gui_context: &mut GuiContext);
}

#[derive(Default)]
pub struct Boundary {
    location: egui::Pos2,
    size: egui::Vec2,
}

impl Boundary {
    #[allow(dead_code)]
    pub fn offset_x(&self) -> f32 {
        self.location.x
    }

    #[allow(dead_code)]
    pub fn offset_y(&self) -> f32 {
        self.location.y
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }
}

impl From<Response> for Boundary {
    fn from(response: Response) -> Self {
        Self {
            location: response.rect.min,
            size: response.rect.size(),
        }
    }
}

#[derive(Default)]
pub struct BoundaryHolder {
    pub menubar: Boundary,
    pub taskbar: Boundary,
    pub modebar: Boundary,
    pub toolbar: Boundary,
    pub settingsbar: Boundary,
}

pub enum Theme {
    Light,
    Dark,
}

pub struct Screen {
    settings: settingsbar::Settingsbar,
    addons: addons::Addons,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
    modebar: modebar::Modebar,
    toolbar: toolbar::Toolbar,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            settings: settingsbar::Settingsbar::new(),
            addons: addons::Addons::new(),
            menubar: menubar::Menubar::new(),
            taskbar: taskbar::Taskbar::new(),
            modebar: modebar::Modebar::new(),
            toolbar: toolbar::Toolbar::new(),
        }
    }
}

impl Component<Screen> for Screen {
    fn show(&mut self, ctx: &egui::Context, gui_context: &mut GuiContext) {
        self.menubar.show(ctx, gui_context);
        self.taskbar.show(ctx, gui_context);

        //self.addons.show(ctx, None, app);

        self.settings.show(ctx, gui_context);
        self.toolbar.show(ctx, gui_context);
        self.modebar.show(ctx, gui_context);

        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            /*
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);
            */

            self.addons.show(ctx, ui, gui_context);
        });

        let mode = *gui_context.application_ctx.mode();

        gui_context
            .application_ctx
            .event_wrapping()
            .register(Item::Mode(Some(mode)));
    }
}
