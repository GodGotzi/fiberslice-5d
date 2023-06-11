use std::collections::HashMap;

use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Ui};

use crate::{utils::Creation, prelude::*};


pub struct Toolbar;

impl Creation for Toolbar {
    fn create() -> Self {
        Self { }
    }
}

impl super::Component<Toolbar> for Toolbar {

    fn show(&mut self, ctx: &egui::Context, 
        _ui: Option<&mut Ui>,
        _mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<super::Interface>,          
        gui_events: &mut HashMap<super::ItemType, AsyncPacket<super::Item>>
    ) {

        let response = egui::SidePanel::left("toolbar")
            .resizable(false)
            .default_width(35.0)
            .show(ctx, |ui| {

                AsyncWrapper::<ItemType, Item>::register(
                    ItemType::ToolbarWidth, 
                    Item::ToolbarWidth(ui.available_width()), 
                    gui_events);
                
            }).response;

        let rect = response.rect;

        gui_interface.register_boundary(
            super::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }
}
