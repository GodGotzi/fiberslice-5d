use std::fmt::Display;

use bevy::{window::Window, prelude::Query};
use bevy_egui::{EguiContexts, egui::{Order, LayerId, Id}};

pub struct EguiData {
    touch: bool    
}

impl EguiData {

    pub fn new() -> Self {
        Self {
            touch: false
        }
    }

    pub fn is_touch(&self) -> bool {
        self.touch
    }

    pub fn check_touch(
        &mut self, 
        mut contexts: EguiContexts, 
        mut windows: Query<&mut Window>
    ) {
        let ctx = contexts.ctx_mut();

        let pointer_pos = ctx.input(|i| i.pointer.interact_pos());
        if let Some(pointer_pos) = pointer_pos {
            if let Some(layer) = ctx.layer_id_at(pointer_pos) {
                println!("{}", layer.order.short_debug_format());
            } else {
                self.touch = false
            }
        } else {
            self.touch = false
        }
    }

}