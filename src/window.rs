use winit::{
    dpi,
    error::OsError,
    event_loop::*,
    window::{Icon, Window, WindowAttributes},
};

use crate::config;

pub fn create_window(event_loop: &ActiveEventLoop) -> Result<Window, OsError> {
    let window_icon = load_icon("assets/icons/main_icon.png");

    let attributes = WindowAttributes::default()
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

    event_loop.create_window(attributes)
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
