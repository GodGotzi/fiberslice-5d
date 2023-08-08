use winit::{event_loop::*, window::Window};

use crate::{config, prelude::Error};

pub fn build_window(event_loop: &EventLoop<()>) -> Result<Window, Error> {
    #[cfg(not(target_arch = "wasm32"))]
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("FiberSlice-5D")
        .with_visible(false)
        .with_min_inner_size(config::default::WINDOW_S);

    #[cfg(target_arch = "wasm32")]
    let window_builder = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        winit::window::WindowBuilder::new()
            .with_canvas(Some(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("canvas")
                    .item(0)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap(),
            ))
            .with_inner_size(config::default::WINDOW_S)
            .with_prevent_default(true)
    };

    window_builder
        .build(event_loop)
        .map_err(|_| Error::InitialBuild("error while building window".into()))
}
