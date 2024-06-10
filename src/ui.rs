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

use std::{sync::atomic::AtomicBool, time::Instant};

use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::{Platform, PlatformDescriptor};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use screen::Screen;

use egui::{FontDefinitions, Visuals};

use crate::{
    environment::view::Mode,
    prelude::{Adapter, Error, FrameHandle, Shared, SharedMut, WgpuContext},
    GlobalState, RootEvent,
};

use self::boundary::Boundary;

#[derive(Debug, Clone)]
pub enum UiEvent {}

#[derive(Debug, Clone)]
pub struct UiState {
    pub pointer_in_use: Shared<AtomicBool>,
    pub theme: Theme,
    pub mode: Mode,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            pointer_in_use: Shared::from_inner(AtomicBool::new(false)),
            theme: Theme::Light,
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

pub struct UiUpdateOutput {
    pub paint_jobs: Vec<egui::ClippedPrimitive>,
    pub tdelta: egui::TexturesDelta,
    pub screen_descriptor: ScreenDescriptor,
    pub viewport: (f32, f32, f32, f32),
}

pub struct UiAdapter {
    state: UiState,
    screen: Screen,
    platform: Platform,
}

impl UiAdapter {
    fn update(
        &mut self,
        event: &winit::event::Event<RootEvent>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
    ) {
        self.platform.handle_event(event);
        self.platform
            .update_time(start_time.elapsed().as_secs_f64());

        let (x, y, width, height) = self.screen.construct_viewport(wgpu_context);

        let is_pointer_over_viewport = {
            if let Some(pos) = self.platform.context().pointer_latest_pos() {
                pos.x >= x && pos.x <= x + width && pos.y >= y && pos.y <= y + height
            } else {
                false
            }
        };

        self.state.pointer_in_use.inner().store(
            self.platform.context().is_using_pointer() || !is_pointer_over_viewport,
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}

impl<'a> FrameHandle<'a, RootEvent, Option<UiUpdateOutput>, GlobalState<RootEvent>> for UiAdapter {
    fn handle_frame(
        &'a mut self,
        event: &winit::event::Event<RootEvent>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        global_state: GlobalState<RootEvent>,
    ) -> Result<Option<UiUpdateOutput>, Error> {
        puffin::profile_function!();

        self.update(event, start_time, wgpu_context);

        if let winit::event::Event::WindowEvent { event, .. } = event {
            if event == &winit::event::WindowEvent::RedrawRequested {
                self.platform.begin_frame();

                self.screen.show(
                    &self.platform.context(),
                    &mut UiData::new(SharedMut::from_inner(self.state.clone()), global_state),
                );

                let full_output = self.platform.end_frame(Some(&wgpu_context.window));

                let viewport = self.screen.construct_viewport(wgpu_context);

                let paint_jobs = self
                    .platform
                    .context()
                    .tessellate(full_output.shapes, full_output.pixels_per_point);

                let tdelta: egui::TexturesDelta = full_output.textures_delta;

                let screen_descriptor = ScreenDescriptor {
                    physical_width: wgpu_context.surface_config.width,
                    physical_height: wgpu_context.surface_config.height,
                    scale_factor: wgpu_context.window.scale_factor() as f32,
                };

                if self.platform.context().has_requested_repaint() {
                    wgpu_context.window.request_redraw();
                }

                return Ok(Some(UiUpdateOutput {
                    paint_jobs,
                    tdelta,
                    screen_descriptor,
                    viewport,
                }));
            }
        }

        Ok(None)
    }
}

impl<'a> Adapter<'a, RootEvent, UiState, Option<UiUpdateOutput>, GlobalState<RootEvent>, UiEvent>
    for UiAdapter
{
    fn from_context(context: &WgpuContext) -> (UiState, Self) {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: context.window.inner_size().width,
            physical_height: context.window.inner_size().height,
            scale_factor: context.window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        egui_extras::install_image_loaders(&platform.context());

        let state = UiState::default();
        let screen = Screen::new();

        // We use the egui_wgpu_backend crate as the render backend.

        (
            state.clone(),
            Self {
                state,
                screen,
                platform,
            },
        )
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
pub enum Theme {
    Light,
    Dark,
}

pub struct UiData {
    state: SharedMut<UiState>,
    pub global: GlobalState<RootEvent>,
    // _phantom: std::marker::PhantomData<&'a SharedState>,
}

impl UiData {
    pub fn new(state: SharedMut<UiState>, global: GlobalState<RootEvent>) -> Self {
        Self { state, global }
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
