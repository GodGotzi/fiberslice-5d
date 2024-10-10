use egui::{Context, Ui};
use parking_lot::RwLock;

use crate::{GlobalState, RootEvent};

use super::UiState;

type Content = Box<dyn Fn(&mut Ui, &(UiState, GlobalState<RootEvent>))>;

pub struct Modal {
    content: Content,
}

pub struct ModalWindow {
    modal: Option<Modal>,
}

impl ModalWindow {
    pub fn new() -> Self {
        Self { modal: None }
    }

    pub fn set_modal(&mut self, content: Content) {
        self.modal = Some(Modal { content });
    }

    pub fn show(&mut self, ctx: &Context, state: &(UiState, GlobalState<RootEvent>)) {
        if let Some(modal) = &self.modal {
            egui::Window::new("Modal")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    (modal.content)(ui, state);
                });
        }
    }
}
