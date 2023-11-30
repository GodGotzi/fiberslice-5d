use tokio::task::JoinHandle;

pub struct RenderThread {
    handle: JoinHandle<()>,
}

impl RenderThread {
    pub fn new() -> Self {
        Self {
            handle: tokio::spawn(Self::run()),
        }
    }
}

impl RenderThread {
    pub async fn run() {}
}
