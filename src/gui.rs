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

use three_d::egui;

use crate::{application::Application, prelude::*};

use self::components::addons;

pub trait Component<T> {
    fn show(&mut self, ctx: &egui::Context, app: &mut Application);
}

pub trait InnerComponent<T> {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, app: &mut Application);
}

pub struct Boundary {
    pub location: egui::Vec2,
    pub size: egui::Vec2,
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
    icontable: icon::IconTable,
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
            icontable: icon::IconTable::new(),
        }
    }
}

impl Component<Screen> for Screen {
    fn show(&mut self, ctx: &egui::Context, app: &mut Application) {
        self.menubar.show(ctx, app);
        self.taskbar.show(ctx, app);

        //self.addons.show(ctx, None, app);

        self.settings.show(ctx, app);
        self.toolbar.show(ctx, app);
        self.modebar.show(ctx, app);

        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);

            self.addons.show(ctx, ui, app);
        });

        app.event_wrapping()
            .register(Item::Mode(Some(app.mode().clone())));
    }
}
