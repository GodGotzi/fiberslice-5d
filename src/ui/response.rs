use std::{any::TypeId, collections::HashMap};

use three_d::egui::Response;

use strum::EnumCount;

pub trait Responsive {
    fn empty() -> Self;
    fn clicked(&self) -> bool;
    fn hovered(&self) -> bool;
    fn update(&mut self, response: &Response);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonResponse {
    clicked: bool,
    hovered: bool,
}

impl Responsive for ButtonResponse {
    fn empty() -> Self {
        Self {
            clicked: false,
            hovered: false,
        }
    }

    fn clicked(&self) -> bool {
        self.clicked
    }

    fn hovered(&self) -> bool {
        self.hovered
    }

    fn update(&mut self, response: &Response) {
        self.clicked = response.clicked();
        self.hovered = response.hovered();
    }
}

pub struct Responses {
    pub button_responses: HashMap<TypeId, Vec<ButtonResponse>>,
}

impl Responses {
    pub fn new() -> Self {
        Self {
            button_responses: HashMap::new(),
        }
    }

    pub fn add_button_response<T: 'static + EnumCount>(&mut self) {
        self.button_responses
            .insert(TypeId::of::<T>(), vec![ButtonResponse::empty(); T::COUNT]);
    }

    pub fn get_button_response<T: 'static + Into<usize>>(&self, t: T) -> Option<&ButtonResponse> {
        if let Some(responses) = self.button_responses.get(&TypeId::of::<T>()) {
            Some(&responses[t.into()])
        } else {
            None
        }
    }

    pub fn update_button_response<T: 'static + Into<usize>>(&mut self, t: T, response: &Response) {
        if let Some(responses) = self.button_responses.get_mut(&TypeId::of::<T>()) {
            responses[t.into()].update(response);
        }
    }

    pub fn toogle_button_response<T: 'static + Into<usize>>(&mut self, t: T) {
        if let Some(responses) = self.button_responses.get_mut(&TypeId::of::<T>()) {
            responses[t.into()].clicked = !responses[t.into()].clicked;
        }
    }
}
