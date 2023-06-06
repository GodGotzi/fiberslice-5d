/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use std::collections::HashMap;

use bevy::prelude::*;

use crate::view::{ViewInterface};

use self::utils::Creation;
use self::{screen::Screen, gui::GuiInterface};
use self::gui::{GuiComponent, EventType};

use strum::IntoEnumIterator;

pub mod utils;
pub mod screen;
pub mod gui;
mod options;

pub struct EventWrapper<E> {
    event: Option<E>,
    last_state: Option<E>
}

impl <E> EventWrapper<E> {
    pub fn new() -> Self {
        Self {
            event: None,
            last_state: None
        }
    }

    pub fn register(
        event_type: EventType, 
        event: gui::Event, 
        gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>
    ) {
        let event_wrapper = gui_events.get_mut(&event_type).unwrap();
        event_wrapper.event = Some(event);
    }
}

#[derive(Resource)]
pub struct FiberSlice {
    gui_events: HashMap<gui::EventType, EventWrapper<gui::Event>>,
    screen: Screen,
}

impl FiberSlice {

    pub fn new() -> Self {
        let mut map = HashMap::new();
        
        for event_type in gui::EventType::iter() {
            map.insert(event_type, EventWrapper::new());
        }

        Self {
            screen: Screen::create(),
            gui_events: map
        }
    }

    pub fn ui_frame(&mut self, ctx: &bevy_egui::egui::Context, 
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,       
        events: &mut EventWriter<gui::Event>
    ) {
        
        self.screen.show(ctx, None, view_interface, gui_interface, &mut self.gui_events);

        for entry in self.gui_events.iter_mut() {
            let wrapper = entry.1;
            let event = wrapper.event.unwrap();

            if wrapper.last_state.unwrap() != event {
                events.send(event);
            }

            wrapper.last_state = wrapper.event;
        }


    }

}