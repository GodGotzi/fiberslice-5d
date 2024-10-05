use glam::Vec2;
use winit::{event::MouseButton, keyboard::KeyCode};

#[derive(Debug, Clone)]
pub enum Action {
    Mouse(MouseButton),
    Keyboard(KeyCode),
}

#[derive(Debug, Clone)]
pub struct DragEvent {
    pub delta: Vec2,
    pub action: Action,
}

#[derive(Debug, Clone)]
pub struct ClickEvent {
    pub action: Action,
}

#[derive(Debug, Clone)]
pub struct ScrollEvent {
    pub delta: f32,
    pub action: Action,
}

pub trait InteractiveModel {
    fn clicked(&self, event: ClickEvent);
    fn drag(&self, event: DragEvent);
    fn scroll(&self, event: ScrollEvent);
}
