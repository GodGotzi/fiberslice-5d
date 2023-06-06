/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/


use std::collections::HashMap;

use bevy_egui::egui::{self, Ui};

use super::gui;
use super::{gui::{GuiComponent}, utils::Creation, EventWrapper};

#[derive(PartialEq, Clone)]
pub enum Face {
    Normal,
    _Top,
    _Bottom,
    _Right,
    _Left,
    _Front,
    _Back
}

pub struct LeftOptionPane {
    _face: Option<Face>
}

impl Creation for LeftOptionPane {

    fn create() -> Self {
        Self {
            _face: Some(Face::Normal)
        }
    }

}

impl GuiComponent<LeftOptionPane> for LeftOptionPane {
    fn show(&mut self, _ctx: &egui::Context,
        _ui: Option<&mut Ui>,
        _view_interface: &mut bevy::prelude::ResMut<crate::view::ViewInterface>,
        _gui_interface: &mut bevy::prelude::ResMut<super::gui::GuiInterface>,          
        _gui_events: &mut HashMap<gui::EventType, EventWrapper<gui::Event>>
    ) {
        
    }
}

impl LeftOptionPane {

    pub fn _get_face(&self) -> Option<Face> {
        self._face.clone()
    }

    pub fn _set_face(&mut self, face: Face) {
        self._face = Some(face);
    }
}