use bevy_egui::egui::Response;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonResponse {
    clicked: bool,
    hovered: bool,
}

impl ButtonResponse {
    pub fn new() -> Self {
        Self {
            clicked: false,
            hovered: false,
        }
    }

    pub fn _clicked(&self) -> bool {
        self.clicked
    }

    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn update(&mut self, response: &Response) {
        self.clicked = response.clicked();
        self.hovered = response.hovered();
    }
}