use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::fiberslice::utils::Creation;

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

pub enum Event {
    RequestRedraw,
}

pub struct DefaultRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for DefaultRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

pub fn create_winit_window(event_loop: &EventLoop<Event>) -> Window {
    winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("FiberSlice 5D")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap()
}

impl Creation for EventLoop<Event> {
    fn create() -> Self {
        winit::event_loop::EventLoopBuilder::<Event>::with_user_event().build()
    }
}