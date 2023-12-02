use crate::prelude::SharedMut;

pub struct EventReader<E> {
    events: SharedMut<Vec<E>>,
}

impl<E> Clone for EventReader<E> {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
        }
    }
}

impl<E> EventReader<E> {
    pub fn read(&self) -> Vec<E> {
        let mut events = self.events.lock().unwrap();
        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut events);
        result
    }

    pub fn has_active_events(&self) -> bool {
        let events = self.events.lock().unwrap();
        !events.is_empty()
    }
}

pub struct EventWriter<E> {
    events: SharedMut<Vec<E>>,
}

impl<E> Clone for EventWriter<E> {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
        }
    }
}

impl<E> EventWriter<E> {
    pub fn send(&self, event: E) {
        let mut events = self.events.lock().unwrap();
        events.push(event);
    }
}

pub fn create_event_bundle<T>() -> (EventReader<T>, EventWriter<T>) {
    let events = SharedMut::from_inner(Vec::new());
    let reader = EventReader {
        events: events.clone(),
    };
    let writer = EventWriter { events };
    (reader, writer)
}
