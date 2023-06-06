/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use std::collections::HashMap;

use bevy_egui::egui::{self, Ui};

use crate::{gui, utils::Creation, prelude::{AsyncPacket, Item, ItemType, AsyncWrapper}};

pub struct Addons {
    slider_layer_value: u32,
    slider_time_value: f32 
}

impl Creation for Addons {
    fn create() -> Self {
        Self {
            slider_layer_value: Default::default(),
            slider_time_value: Default::default()
        }
    }
}

impl gui::Component<Addons> for Addons {

    fn show(&mut self, _ctx: &egui::Context,
        ui: Option<&mut Ui>,
        _gui_interface: &mut bevy::prelude::ResMut<gui::Interface>,          
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>
    ) {
        let mut ui = ui.unwrap();

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
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>) {

        ui.horizontal(|ui| {
            let layer_slider = egui::Slider::new(&mut self.slider_layer_value , 
                0..=100);

            ui.add(layer_slider);
        });


        AsyncWrapper::<ItemType, Item>::register(
            ItemType::LayerValue, 
            Item::LayerValue(self.slider_layer_value), 
            gui_events);
    }

    fn show_time_slider(&mut self,  
        ui: &mut &mut Ui, 
        gui_events: &mut HashMap<gui::ItemType, AsyncPacket<gui::Item>>) {
        

        let time_slider = egui::Slider::new(&mut self.slider_time_value, 0.0..=1.0);
    
        ui.add(time_slider);

        AsyncWrapper::<ItemType, Item>::register(
            ItemType::TimeValue, 
            Item::TimeValue(self.slider_time_value), 
            gui_events);
    }
}