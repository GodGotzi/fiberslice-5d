use crate::view::Orientation;
use strum::EnumCount;
use three_d::egui::Response;

use super::data::UiData;

pub trait Responsive {
    fn empty() -> Self;
    fn clicked(&self) -> bool;
    fn hovered(&self) -> bool;
    fn update(&mut self, response: &Response);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonResponse {
    clicked: bool,
    hovered: bool,
}

impl Responsive for ButtonResponse {
    fn empty() -> Self {
        Self {
            clicked: false,
            hovered: false,
        }
    }

    fn clicked(&self) -> bool {
        self.clicked
    }

    fn hovered(&self) -> bool {
        self.hovered
    }

    fn update(&mut self, response: &Response) {
        self.clicked = response.clicked();
        self.hovered = response.hovered();
    }
}

pub struct Responses {
    pub orientation: [ButtonResponse; Orientation::COUNT],
}

impl Responses {
    pub fn new() -> Self {
        Self {
            orientation: [
                ButtonResponse::empty(),
                ButtonResponse::empty(),
                ButtonResponse::empty(),
                ButtonResponse::empty(),
                ButtonResponse::empty(),
                ButtonResponse::empty(),
            ],
        }
    }
}

impl UiData {
    pub fn get_orientation_response(&self, orientation: &Orientation) -> ButtonResponse {
        self.responses.lock().unwrap().orientation[*orientation as usize]
    }

    pub fn update_orientation_response(&mut self, orientation: &Orientation, response: Response) {
        let button_response =
            &mut self.responses.lock().unwrap().orientation[*orientation as usize];

        button_response.clicked = response.clicked();
        button_response.hovered = response.hovered();
    }
}
