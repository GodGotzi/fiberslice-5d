use winit::{
    dpi,
    event_loop::*,
    window::{Icon, Window, WindowBuilder},
};

use crate::{config, prelude::Error};

pub fn build_window<T>(event_loop: &EventLoop<T>) -> Result<Window, Error> {
    let window_icon = load_icon("assets/icons/main_icon.png");

    let window_builder = WindowBuilder::new()
        .with_title("Fiberslice-5D")
        .with_min_inner_size(dpi::LogicalSize::new(
            config::default::WINDOW_S.0,
            config::default::WINDOW_S.1,
        ))
        .with_visible(false)
        .with_resizable(true)
        .with_window_icon(Some(window_icon))
        .with_decorations(true)
        .with_active(true)
        .with_inner_size(dpi::LogicalSize::new(
            config::default::WINDOW_S.0 as f64,
            config::default::WINDOW_S.1 as f64,
        ));

    window_builder
        .build(event_loop)
        .map_err(|e| Error::InitialBuild(e.to_string()))
}

fn load_icon(path: &str) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
