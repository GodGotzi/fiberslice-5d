use std::sync::{Arc, Mutex};

pub struct EventReader<E> {
    events: Arc<Mutex<Vec<E>>>,
}

impl<E> EventReader<E> {
    pub fn read(&mut self) -> Vec<E> {
        let mut events = self.events.lock().unwrap();
        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut events);
        result
    }
}

pub struct EventWriter<E> {
    events: Arc<Mutex<Vec<E>>>,
}

impl<E> EventWriter<E> {
    pub fn write(&mut self, event: E) {
        let mut events = self.events.lock().unwrap();
        events.push(event);
    }
}

pub struct EventHandler<E> {
    reader: EventReader<E>,
    writer: EventWriter<E>,
}

impl<E> EventHandler<E> {
    pub fn new() -> Self {
        let events = Arc::new(Mutex::new(Vec::new()));
        let reader = EventReader {
            events: events.clone(),
        };
        let writer = EventWriter {
            events: events.clone(),
        };
        EventHandler { reader, writer }
    }

    pub fn write(&mut self, event: E) {
        self.writer.write(event);
    }

    pub fn read(&mut self) -> Vec<E> {
        self.reader.read()
    }
}
