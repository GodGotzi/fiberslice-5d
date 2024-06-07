/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

pub mod api;
pub mod boundary;
pub mod components;
pub mod screen;

mod icon;
pub mod visual;

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use screen::Screen;

use egui::{FontDefinitions, Visuals};

use crate::{
    environment::view::Mode,
    event::EventReader,
    prelude::{Adapter, Error, FrameHandle, SharedMut, SharedState, WgpuContext},
};

use self::boundary::Boundary;

#[derive(Debug, Clone)]
pub enum UiEvent {}

pub struct UiAdapter {
    event_reader: EventReader<UiEvent>,

    state: SharedMut<UiState>,
    screen: Screen,
    egui_rpass: RenderPass,
    platform: Platform,
}

impl UiAdapter {
    pub fn share_state(&self) -> SharedMut<UiState> {
        self.state.clone()
    }
}

impl FrameHandle<(), (), ()> for UiAdapter {
    fn handle_frame(
        &mut self,
        event: &winit::event::Event<()>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        context: (),
    ) -> Result<(), Error> {
        puffin::profile_function!();

        self.platform.handle_event(event);

        if *event == winit::event::Event::UserEvent(()) {
            self.platform
                .update_time(start_time.elapsed().as_secs_f64());

            let output_frame = match wgpu_context.surface.get_current_texture() {
                Ok(frame) => frame,
                Err(wgpu::SurfaceError::Outdated) => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                    return Err(Error::Generic("Surface outdated".to_string()));
                }
                Err(e) => {
                    eprintln!("Dropped frame with error: {}", e);
                    return Err(Error::Generic("Dropped frame".to_string()));
                }
            };
            let output_view = output_frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.platform.begin_frame();

            self.screen.show(
                &self.platform.context(),
                &mut UiData::new(self.state.clone()),
            );

            let full_output = self.platform.end_frame(Some(&wgpu_context.window));

            let paint_jobs = self
                .platform
                .context()
                .tessellate(full_output.shapes, full_output.pixels_per_point);

            let mut encoder =
                wgpu_context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("egui_encoder"),
                    });

            let screen_descriptor = ScreenDescriptor {
                physical_width: wgpu_context.surface_config.width,
                physical_height: wgpu_context.surface_config.height,
                scale_factor: wgpu_context.window.scale_factor() as f32,
            };

            let tdelta: egui::TexturesDelta = full_output.textures_delta;

            self.egui_rpass
                .add_textures(&wgpu_context.device, &wgpu_context.queue, &tdelta)
                .expect("add texture ok");

            self.egui_rpass.update_buffers(
                &wgpu_context.device,
                &wgpu_context.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            self.egui_rpass.execute(
                &mut encoder,
                &output_view,
                &paint_jobs,
                &screen_descriptor,
                None,
            );

            wgpu_context.queue.submit(std::iter::once(encoder.finish()));

            output_frame.present();

            self.egui_rpass
                .remove_textures(tdelta)
                .expect("remove texture ok");

            if self.platform.context().has_requested_repaint() {
                wgpu_context.window.request_redraw();
            }
        }

        // let camera_viewport = self.screen.construct_viewport(frame_input);
        // let output = self.ui.construct_output(camera_viewport);

        Ok(())
        // Ok(output)
    }
}

impl Adapter<(), (), (), UiEvent> for UiAdapter {
    fn from_context(context: &WgpuContext) -> (crate::event::EventWriter<UiEvent>, Self) {
        let (reader, writer) = crate::event::create_event_bundle::<UiEvent>();
        let platform = Platform::new(PlatformDescriptor {
            physical_width: context.window.inner_size().width,
            physical_height: context.window.inner_size().height,
            scale_factor: context.window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        egui_extras::install_image_loaders(&platform.context());

        let state = SharedMut::from_inner(UiState::new());
        let screen = Screen::new();

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&context.device, context.surface_format, 1);

        let adapter = Self {
            event_reader: reader,
            state,
            screen,
            egui_rpass,
            platform,
        };

        (writer, adapter)
    }

    fn get_reader(&self) -> &EventReader<UiEvent> {
        &self.event_reader
    }

    fn handle_event(&mut self, event: UiEvent) {
        puffin::profile_function!();
    }

    fn get_adapter_description(&self) -> String {
        "UiAdapter".to_string()
    }
}

impl From<&Theme> for Visuals {
    fn from(theme: &Theme) -> Self {
        match *theme {
            Theme::Light => Visuals::light(),
            Theme::Dark => Visuals::dark(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub theme: Theme,
    pub mode: Mode,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            theme: Theme::Dark,
            mode: Mode::Preview,
        }
    }
}

impl UiState {
    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }
}

#[derive(Debug, Clone)]
pub struct UiResult {
    pub pointer_use: Option<bool>,
}

impl UiResult {
    fn empty() -> Self {
        Self { pointer_use: None }
    }
}

#[derive(Debug, Clone)]
pub enum Theme {
    Light,
    Dark,
}

pub struct UiData<'a> {
    state: SharedMut<UiState>,
    _phantom: std::marker::PhantomData<&'a SharedState>,
}

impl<'a> UiData<'a> {
    pub fn new(state: SharedMut<UiState>) -> Self {
        Self {
            state,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn borrow_ui_state(&mut self) -> RwLockReadGuard<UiState> {
        self.state.read()
    }

    pub fn borrow_mut_ui_state(&mut self) -> RwLockWriteGuard<UiState> {
        self.state.write()
    }
}

pub trait Component: Send + Sync {
    fn show(&mut self, ctx: &egui::Context, state: &mut UiData);

    fn get_enabled_mut(&mut self) -> &mut bool;

    fn get_boundary(&self) -> &Boundary;
}

pub trait InnerComponent: Send + Sync {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, state: &mut UiData);

    fn get_enabled_mut(&mut self) -> &mut bool;

    fn get_boundary(&self) -> &Boundary;
}
