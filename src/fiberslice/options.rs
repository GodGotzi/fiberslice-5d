/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/


use bevy_egui::egui;

use super::{gui::GuiComponent, utils::Creation};

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
        _view_interface: &mut bevy::prelude::ResMut<crate::view::ViewInterface>,
        _gui_interface: &mut bevy::prelude::ResMut<super::gui::GuiInterface>,          
        _events_resize: &mut bevy::prelude::EventWriter<super::gui::GuiResizeEvent>
    ) {
        todo!()
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