use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use atomic_float::AtomicF32;
use parking_lot::RwLock;

use crate::{prelude::Shared, ui::custom_toasts::MODEL_LOAD_PROGRESS, GLOBAL_STATE};

#[derive(Debug)]
pub struct Process {
    task: RwLock<String>,
    progress: AtomicF32,
    finished: AtomicBool,
    closed: AtomicBool,
}

impl Process {
    pub fn new() -> Self {
        Self {
            task: RwLock::new(String::new()),
            progress: AtomicF32::new(0.0),
            finished: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        }
    }

    pub fn set_progress(&self, progress: f32) {
        self.progress
            .store(progress, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_task(&self, task: String) {
        *self.task.write() = task;
    }

    pub fn get(&self) -> f32 {
        self.progress.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn finish(&self) {
        self.finished
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn close(&self) {
        self.closed
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn task(&self) -> String {
        self.task.read().clone()
    }

    pub fn is_finished(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug, Default)]
pub struct ProcessTracker {
    map: HashMap<u32, HashMap<String, Arc<Process>>>,
}

impl ProcessTracker {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        map.insert(MODEL_LOAD_PROGRESS, HashMap::new());

        Self { map }
    }

    pub fn update(&mut self) {
        self.map.values_mut().for_each(|processes| {
            processes.retain(|_, process| !process.is_closed());
        });
    }

    pub fn add(&mut self, id: u32, name: String) -> Shared<Process> {
        let process = Shared::new(Process::new());

        let global_state = GLOBAL_STATE.read();
        let global_state = global_state.as_ref().unwrap();

        self.map
            .entry(id)
            .or_default()
            .insert(name.clone(), process.clone());

        global_state
            .ui_event_writer
            .send(crate::ui::UiEvent::ShowProgressBar(id, name));

        global_state.window.request_redraw();

        process
    }

    pub fn get(&self, id: u32, name: &str) -> Option<&Arc<Process>> {
        self.map.get(&id)?.get(name)
    }
}
