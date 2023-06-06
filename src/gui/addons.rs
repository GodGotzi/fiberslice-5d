/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use std::collections::HashMap;

use bevy_egui::egui::{self, Ui};

use crate::{gui, utils::Creation, prelude::{AsyncPacket, Item, ItemType, AsyncWrapper}};

pub struct Addons;

impl Creation for Addons {
    fn create() -> Self {
        Self {
        }
    }
}

impl gui::Component<Addons> for Addons {

    fn show(&mut self, _ctx: &egui::Context,
        ui_op: Option<&mut Ui>,
        _gui_interface: &mut bevy::prelude::ResMut<gui::Interface>,          
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
    ) {
        let mut ui = ui_op.unwrap();

        self.show_layer_slider(&mut ui, gui_events);
        self.show_time_slider(&mut ui, gui_events);

        /*
        egui::Window::new("Test")
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.label("Label test");
            let _button = ui.button("button test");
        });
        */
    }

}

impl Addons {

    fn show_layer_slider(&mut self, 
        ui: &mut &mut Ui,
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
    ) {

        ui.horizontal(|ui| {
            AsyncWrapper::<ItemType, Item>::register_with_ref(
                Item::LayerValue(Default::default()), 
                ItemType::LayerValue, |item, ui| {
                    if let Item::LayerValue(width) = item {
                        let layer_slider = egui::Slider::new(width , 0..=100);
            
                        ui.add(layer_slider);
                    } 
                }, ui, gui_events);
        });

    }

    fn show_time_slider(&mut self,  
        ui: &mut &mut Ui, 
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
    ) {

        AsyncWrapper::<ItemType, Item>::register_with_ref(
            Item::TimeValue(Default::default()), 
            ItemType::TimeValue, |item, ui| {
                if let Item::TimeValue(width) = item {
                    let time_slider = egui::Slider::new(width, 0.0..=1.0);  

                    ui.add(time_slider); 
                } 
            }, ui, gui_events);

    }
}