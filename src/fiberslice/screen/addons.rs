/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use std::collections::HashMap;

use bevy_egui::egui::{self, Ui};

use crate::fiberslice::{gui::{GuiComponent, self}, utils::Creation, EventWrapper};

pub struct ViewAddons {
    slider_layer_value: u32,
    slider_time_value: f32 
}

impl Creation for ViewAddons {
    fn create() -> Self {
        Self {
            slider_layer_value: Default::default(),
            slider_time_value: Default::default()
        }
    }
}

impl GuiComponent<ViewAddons> for ViewAddons {

    fn show(&mut self, _ctx: &egui::Context,
        ui: Option<&mut Ui>,
        view_interface: &mut bevy::prelude::ResMut<crate::view::ViewInterface>,
        _gui_interface: &mut bevy::prelude::ResMut<crate::fiberslice::gui::GuiInterface>,          
        gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>
    ) {
        let mut ui = ui.unwrap();

        self.show_layer_slider(&mut ui, view_interface, gui_events);
        self.show_time_slider(&mut ui, view_interface, gui_events);

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

impl ViewAddons {

    fn show_layer_slider(&mut self, 
        ui: &mut &mut Ui,
        view_interface: &mut bevy::prelude::ResMut<crate::view::ViewInterface>,
        gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>) {

        ui.horizontal(|ui| {
            let layer_slider = egui::Slider::new(&mut self.slider_layer_value , 
                0..=view_interface.preview.layer_amount.unwrap());

            ui.add(layer_slider);
        });


        EventWrapper::<gui::Event>::register(
            gui::EventType::LayerSliderChanged, 
            gui::Event::LayerSliderChanged(self.slider_layer_value), 
            gui_events);
    }

    fn show_time_slider(&mut self,  
        ui: &mut &mut Ui, 
        view_interface: &mut bevy::prelude::ResMut<crate::view::ViewInterface>,
        gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>) {
        
        let time_slider = egui::Slider::new(&mut self.slider_time_value, 0.0..=1.0);
    
        ui.add(time_slider);

        EventWrapper::<gui::Event>::register(
            gui::EventType::TimeSliderChanged, 
            gui::Event::TimeSliderChanged(self.slider_time_value), 
            gui_events);
    }
}