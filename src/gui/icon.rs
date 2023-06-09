
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
        let whole_path = Self::format_icon_path(path);

        let image = match image::io::Reader::open(&whole_path) {
            Ok(img) => match img.decode() {
                Ok(img) => img,
                Err(error) => {
                    panic!("Error while opening icon: {}, Error: {}", &whole_path, error);
                }
            },
            Err(error) => {
                panic!("Error while opening icon: {}, Error: {}", &whole_path, error);
            }
        };

        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        Some(RetainedImage::from_color_image (&whole_path, color_image))
    }

    fn format_icon_path(icon_path: &str) -> String {
        format!("assets\\icons\\{}", icon_path)
    }

}