use three_d::{FrameInput, FrameInputGenerator, WindowSettings, WindowedContext};
use winit::{
    dpi,
    event::WindowEvent,
    event_loop::*,
    window::{Icon, Window, WindowBuilder},
};

use crate::{config, prelude::Error};

pub struct WindowHandler {
    window: Window,
    context: WindowedContext,
    frame_input_generator: FrameInputGenerator,
}

impl WindowHandler {
    pub fn from_event_loop(event_loop: &EventLoop<()>) -> Self {
        let window = build_window(event_loop).unwrap();
        let context = WindowedContext::from_winit_window(&window, Default::default()).unwrap();
        let frame_input_generator = FrameInputGenerator::from_winit_window(&window);

        Self {
            window,
            context,
            frame_input_generator,
        }
    }

    pub fn handle_winit_event(&mut self, event: &WindowEvent<'_>, control_flow: &mut ControlFlow) {
        self.frame_input_generator.handle_winit_window_event(event);

        match event {
            winit::event::WindowEvent::Resized(physical_size) => {
                self.context.resize(*physical_size);
            }
            winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.context.resize(**new_inner_size);
            }
            winit::event::WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            _ => (),
        }
    }

    pub fn next_frame_input(&mut self) -> FrameInput {
        self.frame_input_generator.generate(&self.context)
    }

    pub fn init(&mut self) {
        self.window.set_visible(true);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn borrow_context(&self) -> &WindowedContext {
        &self.context
    }
}

pub fn build_window(event_loop: &EventLoop<()>) -> Result<Window, Error> {
    let window_icon = load_icon("assets/icons/main_icon.png");
    let window_settings = WindowSettings {
        title: "Fiberslice-5D".to_string(),
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
            .with_visible(false)
            .with_resizable(true)
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
