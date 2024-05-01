use egui::Response;
use three_d::egui;

#[derive(Debug, Default, Clone, Copy)]
pub struct Boundary {
    pub location: egui::Pos2,
    size: egui::Vec2,
}

impl Boundary {
    pub fn new(location: egui::Pos2, size: egui::Vec2) -> Self {
        Self { location, size }
    }

    pub fn zero() -> Self {
        Self {
            location: egui::Pos2::ZERO,
            size: egui::Vec2::ZERO,
        }
    }

    pub fn get_width(&self) -> f32 {
        self.size.x
    }

    pub fn get_height(&self) -> f32 {
        self.size.y
    }

    pub fn get_size(&self) -> egui::Vec2 {
        self.size
    }
}

impl From<Response> for Boundary {
    fn from(response: Response) -> Self {
        Self {
            location: response.rect.min,
            size: response.rect.size(),
        }
    }
}
