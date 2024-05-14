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
mod response;
mod visual;

use std::borrow::Borrow;

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use three_d::{
    egui::{self, Visuals},
    Context, FrameInput,
};

use crate::{
    environment::view::{Mode, Orientation},
    event::EventReader,
    prelude::{Adapter, Error, FrameHandle, SharedMut, SharedState},
};

use self::{
    boundary::Boundary, parallel::ParallelUiOutput, response::Responses,
    visual::customize_look_and_feel,
};

#[derive(Debug, Clone)]
pub enum UiEvent {}

pub struct UiAdapter {
    state: SharedMut<UiState>,
    event_reader: EventReader<UiEvent>,

    tx_frame_input: Option<tokio::sync::watch::Sender<Option<FrameInput>>>,
    rx_result: Option<tokio::sync::watch::Receiver<Option<ParallelUiOutput>>>,
}

impl UiAdapter {
    pub fn share_state(&self) -> SharedMut<UiState> {
        self.state.clone()
    }

    pub fn init(&mut self, shared_state: &SharedState) {
        let (tx_frame_input, rx_frame_input) = tokio::sync::watch::channel(None);
        let (tx_result, rx_result) = tokio::sync::watch::channel(None);

        self.tx_frame_input = Some(tx_frame_input);
        self.rx_result = Some(rx_result);

        tokio::spawn(Self::renderer(
            rx_frame_input,
            tx_result,
            self.state.clone(),
            shared_state.clone(),
        ));
    }

    async fn renderer(
        mut rx_frame_input: tokio::sync::watch::Receiver<Option<FrameInput>>,
        tx_result: tokio::sync::watch::Sender<Option<ParallelUiOutput>>,
        state: SharedMut<UiState>,
        shared_state: SharedState,
    ) {
        let mut screen = screen::Screen::new();

        let mut parallel_ui = parallel::ParallelUi::new();

        // println!("Ui renderer started, waiting...");
        while rx_frame_input.changed().await.is_ok() {
            let now = std::time::Instant::now();
            // println!("Ui renderer got new frame input");
            let frame_input = rx_frame_input.borrow().as_ref().unwrap().clone();

            parallel_ui.update(
                &mut frame_input.events.clone(),
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |ctx| {
                    let mut result = UiResult::empty();
                    result.pointer_use = Some(ctx.is_using_pointer());

                    let visuals = customize_look_and_feel((&state.read().theme).into());
                    ctx.set_visuals(visuals);

                    screen.show(
                        ctx,
                        &mut UiData {
                            state: state.clone(),
                            shared_state: &shared_state,
                        },
                    );
                },
            );

            let camera_viewport = screen.construct_viewport(&frame_input);
            let output = parallel_ui.construct_output(camera_viewport);
            tx_result.send(Some(output)).unwrap();

            println!("Ui updating took {:?}", now.elapsed());
        }
    }
}

impl FrameHandle<ParallelUiOutput, &SharedState> for UiAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &three_d::FrameInput,
        _context: &SharedState,
    ) -> Result<ParallelUiOutput, Error> {
        if let Some(tx_frame_input) = self.tx_frame_input.as_ref() {
            tx_frame_input.send(Some(frame_input.clone())).unwrap();
        }

        if let Some(rx) = self.rx_result.as_mut() {
            if let Some(result) = rx.borrow_and_update().as_ref() {
                return Ok(result.clone());
            }
        }

        Err(Error::UiNotRendered)
    }
}

impl Adapter<ParallelUiOutput, &SharedState, UiEvent> for UiAdapter {
    fn from_context(_context: &Context) -> (crate::event::EventWriter<UiEvent>, Self) {
        let (reader, writer) = crate::event::create_event_bundle::<UiEvent>();
        let state = SharedMut::from_inner(UiState::new());

        state.write().responses.add_button_response::<Orientation>();

        let instance = Self {
            state,
            event_reader: reader,

            tx_frame_input: None,
            rx_result: None,
        };

        (writer, instance)
    }

    fn get_reader(&self) -> &EventReader<UiEvent> {
        &self.event_reader
    }

    fn handle_event(&mut self, event: UiEvent) {}

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

    pub responses: Responses,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,

            responses: Responses::new(),
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
    shared_state: &'a SharedState,
}

impl<'a> UiData<'a> {
    pub fn borrow_ui_state(&mut self) -> RwLockReadGuard<UiState> {
        self.state.read()
    }

    pub fn borrow_mut_ui_state(&mut self) -> RwLockWriteGuard<UiState> {
        self.state.write()
    }

    pub fn borrow_shared_state(&self) -> &'a SharedState {
        self.shared_state
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
