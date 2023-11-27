use std::time::Instant;

pub use crate::error::Error;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Resource)]
pub struct Context {
    now: Instant,
    latest: Instant,
}

impl Context {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            latest: Instant::now(),
        }
    }

    pub fn fps(&self) -> f32 {
        1.0 / (self.now - self.latest).as_secs_f32()
    }

    pub fn update(&mut self) {
        self.latest = self.now;
        self.now = Instant::now();
    }
}

fn update_context(mut context: ResMut<Context>) {
    context.update();
}
