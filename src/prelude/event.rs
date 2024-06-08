use std::{any::TypeId, collections::HashMap, fmt::Debug};

use super::SharedMut;

#[derive(Debug)]
pub struct EventWriters<T: 'static + Debug> {
    writers: HashMap<TypeId, EventWriter<T>>,
}

impl<T: 'static + Debug> Default for EventWriters<T> {
    fn default() -> Self {
        Self {
            writers: HashMap::new(),
        }
    }
}

impl<T: 'static + Debug> EventWriters<T> {
    pub fn add_adapter_writer<A: 'static>(&mut self, writer: EventWriter<T>) {
        self.writers.insert(TypeId::of::<A>(), writer);
    }

    pub fn send_event_to_adapter<A: 'static>(&mut self, event: T) {
        if let Some(writer) = self.writers.get_mut(&TypeId::of::<A>()) {
            writer.send(event);
        }
    }
}

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
        let result = self.events.read().clone();
        self.events.write().clear();

        result
    }

    pub fn has_active_events(&self) -> bool {
        !self.events.read().is_empty()
    }
}

#[derive(Debug)]
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
