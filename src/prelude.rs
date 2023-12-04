use std::{
    fmt::Debug,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use three_d::{Context, FrameInput};

pub use crate::error::Error;
use crate::{
    environment::EnvironmentEvent,
    event::{EventReader, EventWriter},
    picking::PickingEvent,
    render::RenderEvent,
    settings::FilamentSettings,
    settings::PrinterSettings,
    settings::SliceSettings,
    ui::UiEvent,
};

#[derive(Default)]
pub struct SharedMut<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for SharedMut<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> SharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, PoisonError<MutexGuard<T>>> {
        self.inner.lock()
    }

    pub fn lock_expect(&self) -> MutexGuard<T> {
        self.inner.lock().expect("Failed to lock shared mut")
    }
}

#[derive(Clone, Default)]
pub struct Shared<T> {
    inner: Arc<T>,
}

impl<T> Shared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

pub trait FrameHandle<T, C> {
    fn handle_frame(&mut self, frame_input: &three_d::FrameInput, context: C) -> Result<T, Error>;
}

pub trait Adapter<T, C, E: Debug>: FrameHandle<T, C> {
    fn from_context(context: &Context) -> (EventWriter<E>, Self);

    fn get_reader(&self) -> &EventReader<E>;
    fn get_adapter_description(&self) -> String;

    fn handle_event(&mut self, event: E);

    fn handle_events(&mut self) {
        if self.get_reader().has_active_events() {
            let events = self.get_reader().read();

            for event in events {
                println!("=================");
                println!("Handling event");
                println!("Adapter: {:?}", self.get_adapter_description());
                println!("Event: {:?}", event);
                println!("=================");

                self.handle_event(event);
            }
        }
    }
}

#[derive(Default)]
pub struct SharedSettings {
    pub slice_settings: SharedMut<SliceSettings>,
    pub printer_settings: SharedMut<PrinterSettings>,
    pub filament_settings: SharedMut<FilamentSettings>,
}

pub struct SharedState {
    frame_input: Option<FrameInput>,

    pub settings: SharedSettings,
    pub writer_ui_event: EventWriter<UiEvent>,
    pub writer_environment_event: EventWriter<EnvironmentEvent>,
    pub writer_render_event: EventWriter<RenderEvent>,
    pub writer_picking_event: EventWriter<PickingEvent>,
}

impl SharedState {
    pub fn new(
        writer_render_event: EventWriter<RenderEvent>,
        writer_environment_event: EventWriter<EnvironmentEvent>,
        writer_ui_event: EventWriter<UiEvent>,
        writer_picking_event: EventWriter<PickingEvent>,
    ) -> Self {
        Self {
            frame_input: None,
            settings: SharedSettings::default(),
            writer_ui_event,
            writer_environment_event,
            writer_render_event,
            writer_picking_event,
        }
    }

    pub fn fps(&self) -> Option<f32> {
        self.frame_input
            .as_ref()
            .map(|frame_input| 1000.0 / frame_input.elapsed_time as f32)
    }
}

impl FrameHandle<(), ()> for SharedState {
    fn handle_frame(&mut self, frame_input: &FrameInput, _context: ()) -> Result<(), Error> {
        self.frame_input = Some(frame_input.clone());

        Ok(())
    }
}
