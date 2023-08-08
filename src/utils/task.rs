use tokio::sync::oneshot::{channel, Receiver};
use tokio::task::JoinHandle;

pub trait Task {
    fn kill(&mut self);
}

#[derive(Debug)]
pub struct TaskWithResult<T> {
    handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<T>>,
}

#[allow(dead_code)]
impl<T: Sync + Send + std::fmt::Debug + 'static> TaskWithResult<T> {
    pub fn new() -> Self {
        Self {
            handle: None,
            receiver: None,
        }
    }

    pub fn run(&mut self, runnable: Box<dyn Fn() -> T + Sync + Send>) {
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
}

impl<T> Task for TaskWithResult<T> {
    fn kill(&mut self) {
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
