use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use three_d::{Context, FrameInput};

pub use crate::error::Error;
use crate::{settings::FilamentSettings, settings::PrinterSettings, settings::SliceSettings};

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

pub trait FrameHandle<T> {
    fn handle_frame(&mut self, frame_input: &three_d::FrameInput) -> Result<T, Error>;
}

pub trait RenderHandle {
    fn handle(&self);
}

pub trait Adapter<T>: FrameHandle<T> {
    fn from_context(context: &Context) -> Self;
}

#[derive(Default)]
pub struct SharedSettings {
    slice_settings: SharedMut<SliceSettings>,
    printer_settings: SharedMut<PrinterSettings>,
    filament_settings: SharedMut<FilamentSettings>,
}

#[derive(Default)]
pub struct SharedState {
    frame_input: Option<FrameInput>,
    settings: SharedSettings,
}

impl SharedState {
    pub fn fps(&self) -> Option<f32> {
        if let Some(frame_input) = self.frame_input.as_ref() {
            Some((1000.0 / frame_input.elapsed_time) as f32)
        } else {
            None
        }
    }
}

impl FrameHandle<()> for SharedState {
    fn handle_frame(&mut self, frame_input: &FrameInput) -> Result<(), Error> {
        self.frame_input = Some(frame_input.clone());

        Ok(())
    }
}
