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
mod response;
mod visual;

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use three_d::{
    egui::{self, Visuals},
    Context, FrameInput, GUI,
};

use crate::{
    event::EventReader,
    prelude::{Adapter, Error, FrameHandle, SharedState},
    tools::Tool,
    view::{Mode, Orientation},
};

use strum::EnumCount;

use self::{boundary::Boundary, response::Responses, visual::customize_look_and_feel};

#[derive(Debug)]
pub enum UiEvent {}

pub struct UiAdapter {
    gui: GUI,
    screen: screen::Screen,
    state: Rc<RefCell<UiState>>,
    event_reader: EventReader<UiEvent>,
}

impl UiAdapter {
    pub fn borrow_gui(&self) -> &GUI {
        &self.gui
    }

    pub fn share_state(&self) -> Rc<RefCell<UiState>> {
        self.state.clone()
    }
}

impl FrameHandle<UiResult, &SharedState> for UiAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        shared_state: &SharedState,
    ) -> Result<UiResult, Error> {
        let mut result = UiResult::empty();

        self.state.borrow_mut().components.delete_cache();

        self.gui.update(
            &mut frame_input.events.clone(),
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                result.pointer_use = Some(ctx.is_using_pointer());

                let visuals = customize_look_and_feel((&self.state.borrow().theme).into());
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

        Ok(result)
    }
}

impl Adapter<UiResult, &SharedState, UiEvent> for UiAdapter {
    fn from_context(context: &Context) -> (crate::event::EventWriter<UiEvent>, Self) {
        let (reader, writer) = crate::event::create_event_bundle::<UiEvent>();
        let state = Rc::new(RefCell::new(UiState::new()));

        state
            .borrow_mut()
            .responses
            .add_button_response::<Orientation>();

        (
            writer,
            Self {
                gui: GUI::new(context),
                screen: screen::Screen::new(),
                state,
                event_reader: reader,
            },
        )
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

pub struct UiState {
    pub theme: Theme,
    pub mode: Mode,
    tools_enabled: [bool; Tool::COUNT],

    pub responses: Responses,
    pub components: Components,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
            tools_enabled: [false; Tool::COUNT],

            responses: Responses::new(),
            components: Components::default(),
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

#[derive(Default)]
pub struct Components {
    pub menubar: ComponentData,
    pub taskbar: ComponentData,
    pub modebar: ComponentData,
    pub toolbar: ComponentData,
    pub settingsbar: ComponentData,
    pub addons: ComponentData,
}

impl Components {
    pub fn delete_cache(&mut self) {
        self.menubar.delete_cache();
        self.taskbar.delete_cache();
        self.modebar.delete_cache();
        self.toolbar.delete_cache();
        self.settingsbar.delete_cache();
    }
}

pub struct ComponentData {
    pub boundary: Option<Boundary>,
    pub enabled: bool,
}

impl ComponentData {
    fn delete_cache(&mut self) {
        self.boundary = None;
    }

    pub fn boundary(&self) -> Boundary {
        self.boundary.unwrap_or(Boundary::zero())
    }

    pub fn set_boundary(&mut self, boundary: Boundary) {
        self.boundary = Some(boundary);
    }
}

impl Default for ComponentData {
    fn default() -> Self {
        Self {
            boundary: None,
            enabled: true,
        }
    }
}

pub struct UiResult {
    pub pointer_use: Option<bool>,
}

impl UiResult {
    fn empty() -> Self {
        Self { pointer_use: None }
    }
}

#[derive(Clone)]
pub enum Theme {
    Light,
    Dark,
}

pub struct UiData<'a> {
    state: Rc<RefCell<UiState>>,
    shared_state: &'a SharedState,
}

impl<'a> UiData<'a> {
    pub fn borrow_ui_state(&mut self) -> Ref<UiState> {
        self.state.borrow()
    }

    pub fn borrow_mut_ui_state(&mut self) -> RefMut<UiState> {
        self.state.borrow_mut()
    }

    pub fn borrow_shared_state(&self) -> &'a SharedState {
        self.shared_state
    }
}

pub trait SuperComponent {
    fn show(&mut self, ctx: &egui::Context, state: &mut UiData);
}

pub trait Component {
    fn show(&mut self, ctx: &egui::Context, state: &mut UiData);
}

pub trait InnerComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, state: &mut UiData);
}

pub trait TextComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}

pub trait InnerTextComponent<P> {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, prefix: P, suffix: P);
}
