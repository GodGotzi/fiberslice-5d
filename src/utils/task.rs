use tokio::sync::oneshot::{channel, Receiver};
use tokio::task::JoinHandle;

pub trait VirtualTask {
    fn kill(&mut self);
}

pub struct VirtualResultTask<T> {
    handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<T>>,
}

impl<T: Sync + Send + std::fmt::Debug + 'static> VirtualResultTask<T> {
    pub fn new() -> Self {
        Self {
            handle: None,
            receiver: None,
        }
    }

    pub fn run(&mut self, runnable: fn() -> T) {
        let (sender, receiver) = channel::<T>();
        let handle = tokio::spawn(async move {
            sender.send(runnable()).unwrap();
        });

        self.handle = Some(handle);
        self.receiver = Some(receiver);
    }

    pub fn result(&mut self) -> Option<T> {
        if self.receiver.is_some() {
            if let Ok(result) = self.receiver.as_mut().unwrap().try_recv() {
                return Some(result);
            }
        }

        None
    }

    pub fn kill(&mut self) {
        if self.handle.is_some() {
            self.handle.take().unwrap().abort();
        }

        if self.receiver.is_some() {
            self.receiver.take().unwrap().close();
        }

        self.handle = None;
        self.receiver = None;
    }
}
