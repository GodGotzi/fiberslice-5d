use std::{borrow::Borrow, fmt::Debug, sync::Arc};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use three_d::{Context, FrameInput};

pub use crate::error::Error;
use crate::{
    environment::EnvironmentEvent,
    event::{EventReader, EventWriter},
    picking::PickingEvent,
    render::RenderEvent,
    settings::PrinterSettings,
    settings::{tree::Setting, FilamentSettings},
    ui::UiEvent,
};

#[derive(Default, Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}

#[derive(Clone, Default, Debug)]
pub struct WrappedSharedMut<T: Debug> {
    inner: Arc<RwLock<Wrapper<T>>>,
}

impl<T: Debug> WrappedSharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Wrapper { inner })),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<Wrapper<T>> {
        self.inner.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<Wrapper<T>> {
        self.inner.write()
    }
}

#[derive(Default, Debug)]
pub struct WrappedShared<T: Debug> {
    inner: Arc<Wrapper<T>>,
}

impl<T: Debug> WrappedShared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(Wrapper { inner }),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner.inner
    }
}

#[derive(Default, Debug)]
pub struct SharedMut<T: Debug> {
    inner: Arc<RwLock<T>>,
}

impl<T: Debug> Clone for SharedMut<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug> SharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write()
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

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

pub trait FrameHandle<T, C> {
    fn handle_frame(&mut self, frame_input: &three_d::FrameInput, context: C) -> Result<T, Error>;
}

pub trait Adapter<T, C, E: Debug + Clone>: FrameHandle<T, C> {
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

#[derive(Clone)]
pub struct SharedSettings {
    pub tree_settings: Setting,
    pub printer_settings: SharedMut<PrinterSettings>,
    pub filament_settings: SharedMut<FilamentSettings>,
}

impl Default for SharedSettings {
    fn default() -> Self {
        let tree_settings = Setting::new("settings/main.yaml");
        let printer_settings = SharedMut::from_inner(PrinterSettings::default());
        let filament_settings = SharedMut::from_inner(FilamentSettings::default());

        Self {
            tree_settings,
            printer_settings,
            filament_settings,
        }
    }
}

#[derive(Debug, Default)]
pub struct ToolpathBuffer {}

#[derive(Debug, Default)]
pub struct IntersectionBuffer {}

#[derive(Debug, Default)]
pub struct ControlBuffer {}

#[derive(Clone, Debug, Default)]
pub struct SharedBuffer {
    toolpath: SharedMut<ToolpathBuffer>,
    intersections: SharedMut<IntersectionBuffer>,
    controls: SharedMut<ControlBuffer>,
}

#[derive(Clone)]
pub struct SharedState {
    frame_input: SharedMut<Option<FrameInput>>,

    pub settings: SharedSettings,
    pub shared_buffer: SharedBuffer,

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
            frame_input: SharedMut::from_inner(None),
            settings: SharedSettings::default(),
            shared_buffer: SharedBuffer::default(),
            writer_ui_event,
            writer_environment_event,
            writer_render_event,
            writer_picking_event,
        }
    }

    pub fn fps(&self) -> Option<f32> {
        self.frame_input.read().as_ref().map(|frame_input| {
            println!("Elapsed time: {:?}", frame_input.elapsed_time);
            1000.0 / frame_input.elapsed_time as f32
        })
    }
}

impl FrameHandle<(), ()> for SharedState {
    fn handle_frame(&mut self, frame_input: &FrameInput, _context: ()) -> Result<(), Error> {
        self.frame_input.write().replace(frame_input.clone());

        println!("Frame: {:?}", frame_input);
        println!("Elapsed time: {:?}", frame_input.elapsed_time);

        Ok(())
    }
}
