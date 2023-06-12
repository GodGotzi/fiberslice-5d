/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Color32};
use crate::{prelude::{Item, Mode, AsyncWrapper}, config};

use super::{gui, settingsbar, addons, menubar, taskbar, modebar, toolbar};
use crate::utils::Creation;

pub struct Screen {
    mode: Mode,
    settings: settingsbar::Settingsbar,
    addons: addons::Addons,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
    modebar: modebar::Modebar,
    toolbar: toolbar::Toolbar,
}

impl Screen {

    pub fn get_settingsbar_width(item_wrapper: &mut ResMut<AsyncWrapper>) -> f32 {
        if let Some(item) = item_wrapper.find_packet_mut(Item::SettingsWidth(None)) {
            if item.get_sync().is_some() {
                if let Item::SettingsWidth(Some(width)) = item.get_sync().unwrap() {
                    width
                } else {
                    config::gui::default::SETTINGSBAR_W
                }
            } else {
                config::gui::default::SETTINGSBAR_W
            }
        } else {
            config::gui::default::SETTINGSBAR_W
        }
    }

}

impl Creation for Screen {
    fn create() -> Screen {
        Screen {
            mode: Mode::Prepare,
            settings: settingsbar::Settingsbar::create(),
            addons: addons::Addons::create(),
            menubar: menubar::Menubar::create(),
            taskbar: taskbar::Taskbar::create(),
            modebar: modebar::Modebar::create(),
            toolbar: toolbar::Toolbar::create(),
        }
    }
}

impl gui::Component<Screen> for Screen {

    fn show(&mut self, ctx: &egui::Context,
        _ui: Option<&mut egui::Ui>,
        _mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<gui::Interface>,          
        item_wrapper: &mut ResMut<AsyncWrapper>,
    ) {
        self.menubar.show(ctx, None, Some(&mut self.mode), gui_interface, item_wrapper);
        self.taskbar.show(ctx, None, Some(&mut self.mode), gui_interface, item_wrapper);

        let frame = egui::containers::Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame)
        .show(ctx, |ui| {

            self.addons.show(ctx, Some(ui), Some(&mut self.mode), gui_interface, item_wrapper);
            self.settings.show(ctx, Some(ui), Some(&mut self.mode), gui_interface, item_wrapper);
            self.toolbar.show(ctx, Some(ui), Some(&mut self.mode), gui_interface, item_wrapper);
            self.modebar.show(ctx, Some(ui), Some(&mut self.mode), gui_interface, item_wrapper);
        });

        item_wrapper.register(Item::Mode(Some(self.mode)));
    }
    
}