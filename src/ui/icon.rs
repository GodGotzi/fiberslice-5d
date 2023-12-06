use std::{any::TypeId, collections::HashMap};

use egui_extras::RetainedImage;
use three_d::egui;

use lazy_static::lazy_static;

use crate::view::Orientation;

lazy_static! {
    pub static ref ICONTABLE: IconTable = IconTable::new();
}

pub struct IconTable {
    icons: HashMap<TypeId, Vec<RetainedImage>>,
}

impl IconTable {
    pub fn new() -> Self {
        let mut icons = HashMap::new();

        icons.insert(
            TypeId::of::<Orientation>(),
            vec![
                Self::load_icon("orientation_default_30x30.png").unwrap(),
                Self::load_icon("orientation_default_30x30.png").unwrap(),
                Self::load_icon("orientation_top_30x30.png").unwrap(),
                Self::load_icon("orientation_left_30x30.png").unwrap(),
                Self::load_icon("orientation_right_30x30.png").unwrap(),
                Self::load_icon("orientation_front_30x30.png").unwrap(),
            ],
        );

        Self { icons }
    }

    pub fn get_icon<T: 'static + Into<usize>>(&self, t: T) -> Option<&RetainedImage> {
        if let Some(icons) = self.icons.get(&TypeId::of::<T>()) {
            Some(&icons[t.into()])
        } else {
            None
        }
    }

    fn load_icon(path: &str) -> Option<RetainedImage> {
        let whole_path = Self::format_icon_path(path);

        let image = match image::io::Reader::open(&whole_path) {
            Ok(img) => match img.decode() {
                Ok(img) => img,
                Err(error) => {
                    panic!(
                        "Error while opening icon: {}, Error: {}",
                        &whole_path, error
                    );
                }
            },
            Err(error) => {
                panic!(
                    "Error while opening icon: {}, Error: {}",
                    &whole_path, error
                );
            }
        };

        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();

        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        Some(RetainedImage::from_color_image(&whole_path, color_image))
    }

    fn format_icon_path(icon_path: &str) -> String {
        format!(
            "{}{}",
            uni_path::Path::new("assets/icons/").to_str(),
            icon_path
        )
    }
}
