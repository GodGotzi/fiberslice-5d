use three_d::WindowSettings;
use winit::{
    dpi,
    event_loop::*,
    window::{Icon, Window, WindowBuilder},
};

use crate::{config, prelude::Error};

pub fn build_window(event_loop: &EventLoop<()>) -> Result<Window, Error> {
    let window_icon = load_icon("assets/icons/main_icon.png");
    let window_settings = WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some(config::default::WINDOW_S),
        ..Default::default()
    };

    let window_builder = {
        let window_builder = WindowBuilder::new()
            .with_title(&window_settings.title)
            .with_min_inner_size(dpi::LogicalSize::new(
                window_settings.min_size.0,
                window_settings.min_size.1,
            ))
            .with_transparent(false)
            .with_window_icon(Some(window_icon))
            .with_decorations(!window_settings.borderless);

        if let Some((width, height)) = window_settings.max_size {
            window_builder
                .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                .with_max_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
        } else {
            window_builder.with_maximized(true)
        }
    };

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
