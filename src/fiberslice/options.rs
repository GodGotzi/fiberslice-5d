
use bevy_egui::egui;
use egui::{CollapsingHeader, Context, Frame, Stroke, Ui};

#[derive(PartialEq, Clone)]
pub enum Face {
    NORMAL,
    TOP,
    BOTTOM,
    RIGHT,
    LEFT,
    FRONT,
    BACK
}

pub struct LeftOptionPane {
    face: Option<Face>
}

impl Default for LeftOptionPane {
    fn default() -> Self {
        Self {
            face: Some(Face::NORMAL)
        }
    }
}

impl LeftOptionPane {
    pub fn ui(&mut self, _ctx: &Context, ui: &mut Ui) {
        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings")
                    .show(ui, |ui| {
                        ui.label("test");
                    });
            });
    }

    pub fn get_face(&self) -> Option<Face> {
        self.face.clone()
    }

    pub fn set_face(&mut self, face: Face) {
        self.face = Some(face);
    }
}

/*
struct RightOptionPane {

}
 */