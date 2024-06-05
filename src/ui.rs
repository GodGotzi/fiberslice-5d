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
pub mod parallel;
pub mod visual;

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use three_d::{Context, FrameInput};

use egui::Visuals;

use crate::{
    environment::view::Mode,
    event::EventReader,
    prelude::{Adapter, Error, FrameHandle, SharedMut, SharedState},
};

use self::{
    boundary::Boundary,
    parallel::{ParallelUi, ParallelUiOutput},
    visual::customize_look_and_feel,
};

#[derive(Debug, Clone)]
pub enum UiEvent {}

pub struct UiAdapter {
    ui: ParallelUi,
    screen: screen::Screen,

    state: SharedMut<UiState>,
    event_reader: EventReader<UiEvent>,
}

impl UiAdapter {
    pub fn share_state(&self) -> SharedMut<UiState> {
        self.state.clone()
    }
}

impl FrameHandle<ParallelUiOutput, &SharedState> for UiAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &three_d::FrameInput,
        shared_state: &SharedState,
    ) -> Result<ParallelUiOutput, Error> {
        puffin::profile_function!();

        /*
                self.ui.update(
            &mut frame_input.events.clone(),
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                egui_extras::install_image_loaders(ctx);

                let mut result = UiResult::empty();
                result.pointer_use = Some(ctx.is_using_pointer());

                let visuals = customize_look_and_feel((&self.state.read().theme).into());
                ctx.set_visuals(visuals);

                self.screen.show(
                    ctx,
                    &mut UiData {
                        state: self.state.clone(),
                        shared_state,
                    },
                );
            },
        );
        */

        let camera_viewport = self.screen.construct_viewport(frame_input);
        let output = self.ui.construct_output(camera_viewport);

        Ok(output)
    }
}

impl Adapter<ParallelUiOutput, &SharedState, UiEvent> for UiAdapter {
    fn from_context(_context: &Context) -> (crate::event::EventWriter<UiEvent>, Self) {
        let (reader, writer) = crate::event::create_event_bundle::<UiEvent>();
        let state = SharedMut::from_inner(UiState::new());

        // state.write().responses.add_button_response::<Orientation>();

        let instance = Self {
            ui: ParallelUi::new(),
            screen: screen::Screen::new(),
            state,
            event_reader: reader,
        };

        (writer, instance)
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
