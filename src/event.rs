use std::fmt::Debug;

use crate::prelude::SharedMut;

pub struct EventReader<E: Debug> {
    events: SharedMut<Vec<E>>,
}

impl<E: Debug> Clone for EventReader<E> {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
        }
    }
}

impl<E: Debug + Clone> EventReader<E> {
    pub fn read(&self) -> Vec<E> {
        let mut result = self.events.read().clone();
        self.events.write().clear();

        result
    }

    pub fn has_active_events(&self) -> bool {
        !self.events.read().is_empty()
    }
}

pub struct EventWriter<E: Debug> {
    events: SharedMut<Vec<E>>,
}

impl<E: Debug> Clone for EventWriter<E> {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
        }
    }
}

impl<E: Debug> EventWriter<E> {
    pub fn send(&self, event: E) {
        self.events.write().push(event);
    }
}

pub fn create_event_bundle<T: Debug>() -> (EventReader<T>, EventWriter<T>) {
    let events = SharedMut::from_inner(Vec::new());
    let reader = EventReader {
        events: events.clone(),
    };
    let writer = EventWriter { events };
    (reader, writer)
}
