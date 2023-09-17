use winit::{
    event_loop::*,
    platform::windows::WindowBuilderExtWindows,
    window::{Icon, Window},
};

use crate::{config, prelude::Error};

pub fn build_window(event_loop: &EventLoop<()>) -> Result<Window, Error> {
    let window_icon = load_icon("assets/icon.png");

    #[cfg(not(target_arch = "wasm32"))]
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("FiberSlice-5D")
        .with_visible(false)
        .with_window_icon(Some(window_icon.clone()))
        .with_taskbar_icon(Some(window_icon))
        .with_min_inner_size(config::default::WINDOW_S);

    window_builder
        .build(event_loop)
        .map_err(|_| Error::InitialBuild("error while building window".into()))
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
