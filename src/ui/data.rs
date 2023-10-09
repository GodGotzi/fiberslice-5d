use std::cell::RefCell;

use bevy::prelude::{EventWriter, Res, ResMut, Resource};
use bevy_egui::egui;
use egui::Response;

use crate::{
    prelude::Context,
    settings::{FilamentSettings, PrinterSettings, SliceSettings},
    view::{Mode, Orientation},
};

use super::{
    boundary::Boundary,
    response::{ButtonResponse, Responses, Responsive},
    Theme,
};

pub type UiData<'a> = &'a UiDataBundle<'a>;

pub trait DataBundle<T> {
    fn wrap(objects: T) -> Self;
}

pub struct EventWriterBundle<'a> {
    orientation: RefCell<EventWriter<'a, Orientation>>,
}

impl<'a> DataBundle<EventWriter<'a, Orientation>> for EventWriterBundle<'a> {
    fn wrap(objects: EventWriter<'a, Orientation>) -> Self {
        Self {
            orientation: RefCell::new(objects),
        }
    }
}

pub struct SettingBundle<'a> {
    pub slice: RefCell<ResMut<'a, SliceSettings>>,
    pub filament: RefCell<ResMut<'a, FilamentSettings>>,
    pub printer: RefCell<ResMut<'a, PrinterSettings>>,
}

impl<'a>
    DataBundle<(
        ResMut<'a, SliceSettings>,
        ResMut<'a, FilamentSettings>,
        ResMut<'a, PrinterSettings>,
    )> for SettingBundle<'a>
{
    fn wrap(
        (slice, filament, printer): (
            ResMut<'a, SliceSettings>,
            ResMut<'a, FilamentSettings>,
            ResMut<'a, PrinterSettings>,
        ),
    ) -> Self {
        Self {
            slice: RefCell::new(slice),
            filament: RefCell::new(filament),
            printer: RefCell::new(printer),
        }
    }
}

pub struct UiDataBundle<'a> {
    pub raw: RefCell<ResMut<'a, RawUiData>>,
    pub responses: RefCell<ResMut<'a, Responses>>,
    pub context: Res<'a, Context>,
    pub settings: SettingBundle<'a>,
    writers: EventWriterBundle<'a>,
}

impl<'a>
    DataBundle<(
        ResMut<'a, RawUiData>,
        ResMut<'a, Responses>,
        Res<'a, Context>,
        SettingBundle<'a>,
        EventWriterBundle<'a>,
    )> for UiDataBundle<'a>
{
    fn wrap(
        (raw, responses, context, settings, writers): (
            ResMut<'a, RawUiData>,
            ResMut<'a, Responses>,
            Res<'a, Context>,
            SettingBundle<'a>,
            EventWriterBundle<'a>,
        ),
    ) -> Self {
        Self {
            raw: RefCell::new(raw),
            responses: RefCell::new(responses),
            context,
            settings,
            writers,
        }
    }
}

impl<'a> UiDataBundle<'a> {
    pub fn orienation_writer(&self) -> &RefCell<EventWriter<'a, Orientation>> {
        &self.writers.orientation
    }

    pub fn update_orientation_response(&self, response: &Response, orientation: Orientation) {
        self.responses.borrow_mut().orientation[orientation as usize].update(response);
    }

    pub fn get_orientation_response(&self, orientation: Orientation) -> ButtonResponse {
        self.responses.borrow().orientation[orientation as usize]
    }
}

#[derive(Resource)]
pub struct RawUiData {
    pub theme: Theme,
    pub mode: Mode,
    pub holder: ComponentDataHolder,
}

impl RawUiData {
    pub fn new(theme: Theme, mode: Mode) -> Self {
        Self {
            theme,
            mode,
            holder: ComponentDataHolder::default(),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }
}

pub struct ComponentData {
    boundary: Option<Boundary>,
    pub enabled: bool,
}

impl ComponentData {
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

#[derive(Default)]
pub struct ComponentDataHolder {
    pub menubar: ComponentData,
    pub taskbar: ComponentData,
    pub modebar: ComponentData,
    pub toolbar: ComponentData,
    pub settingsbar: ComponentData,
}

impl ComponentDataHolder {
    pub fn initialized(&self) -> bool {
        self.menubar.boundary.is_some()
            && self.taskbar.boundary.is_some()
            && self.modebar.boundary.is_some()
            && self.toolbar.boundary.is_some()
            && self.settingsbar.boundary.is_some()
    }

    pub fn reset_boundaries(&mut self) {
        self.menubar.boundary = None;
        self.taskbar.boundary = None;
        self.modebar.boundary = None;
        self.toolbar.boundary = None;
        self.settingsbar.boundary = None;
    }
}
