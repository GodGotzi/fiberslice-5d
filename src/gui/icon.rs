
use std::{fs::File, io::Read};

use egui_extras::RetainedImage;

use crate::view::Orientation;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ICONTABLE: IconTable = IconTable::new();
}

pub struct IconTable {
    orientation_default: RetainedImage
}

impl IconTable {

    pub fn new() -> Self {
        Self { 
            orientation_default: Self::load_icon("cube_ico.png").unwrap()
        }
    }

    pub fn get_orientation_icon(&self, orientation: Orientation) -> &RetainedImage {
        match orientation {
            Orientation::Default => &self.orientation_default,
            Orientation::Top => todo!(),
            Orientation::Left => todo!(),
            Orientation::Right => todo!(),
            Orientation::Front => todo!(),
        }
    }

    fn load_icon(path: &str) -> Option<RetainedImage> {
        let mut buffer = vec![];
        let whole_path = Self::format_icon_path(path);
        
        let result = File::open(&whole_path).unwrap().read_to_end(&mut buffer); 

        if let Err(error) = result {
            panic!("Error while opening icon: {}, Error: {}", &whole_path, error);
        } 

        match RetainedImage::from_image_bytes (&whole_path, &buffer[..]) {
            Ok(img) => Some(img),
            Err(error) => {
                println!("Error while opening icon: {}, Error: {}", &whole_path, error);
                None
            }
        }
    }

    fn format_icon_path(icon_path: &str) -> String {
        format!("assets\\icons\\{}", icon_path)
    }

}