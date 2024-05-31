use egui::ImageSource;

use lazy_static::lazy_static;

use crate::environment::view::Orientation;

lazy_static! {
    pub static ref ICONTABLE: IconTable = IconTable::new();
}

pub struct IconTable;

impl IconTable {
    pub fn new() -> Self {
        Self
    }

    pub fn get_orientation_asset(&self, orientation: Orientation) -> ImageSource {
        match orientation {
            Orientation::Default => egui::include_image!("assets/orientation_default_30x30.png"),
            Orientation::Diagonal => egui::include_image!("assets/orientation_default_30x30.png"),
            Orientation::Top => egui::include_image!("assets/orientation_top_30x30.png"),
            Orientation::Left => egui::include_image!("assets/orientation_left_30x30.png"),
            Orientation::Right => egui::include_image!("assets/orientation_right_30x30.png"),
            Orientation::Front => egui::include_image!("assets/orientation_front_30x30.png"),
        }
    }
}
