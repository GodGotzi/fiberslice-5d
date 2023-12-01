use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use three_d::FrameInput;

pub use crate::error::Error;
use crate::{
    environment::Environment, render::RenderState, settings::FilamentSettings,
    settings::PrinterSettings, settings::SliceSettings,
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

pub trait Adapter<T, C>: FrameHandle<T, C> {}

#[derive(Default)]
pub struct SharedSettings {
    pub slice_settings: SharedMut<SliceSettings>,
    pub printer_settings: SharedMut<PrinterSettings>,
    pub filament_settings: SharedMut<FilamentSettings>,
}

pub struct SharedState {
    pub settings: SharedSettings,
    pub render_state: SharedMut<RenderState>,
    pub environment: SharedMut<Environment>,
}

impl FrameHandle<(), ()> for SharedState {
    fn handle_frame(&mut self, _frame_input: &FrameInput, _context: ()) -> Result<(), Error> {
        Ok(())
    }
}
