use bevy::prelude::{Resource, ResMut};

use crate::view::{ViewInterface, self};

use self::screen::Screen;

pub mod utils;
pub mod screen;
mod options;

#[derive(Resource)]
pub struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {

    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }

    pub fn ui_frame(&mut self, ctx: &bevy_egui::egui::Context, view_interface: &mut ResMut<ViewInterface>) {
        self.screen.ui(ctx, view_interface);
    }

}


pub mod gui {
    use bevy::{prelude::{Resource, ResMut}};
    use bevy_egui::EguiContexts;

    
    #[derive(Resource)]
    pub struct GuiInterface {
        touch: bool    
    }

    impl GuiInterface {

        pub fn new() -> Self {
            Self {
                touch: false
            }
        }

        pub fn _is_touch(&self) -> bool {
            self.touch
        }
    }
    
    pub fn check_touch(
        mut contexts: EguiContexts, 
        mut gui_interface: ResMut<GuiInterface>
    ) {
        let ctx = contexts.ctx_mut();

        let pointer_pos = ctx.input(|i| i.pointer.interact_pos());
        if let Some(pointer_pos) = pointer_pos {
            if let Some(layer) = ctx.layer_id_at(pointer_pos) {
                println!("{}", layer.order.short_debug_format());
            } else {
                gui_interface.touch = false;
            }
        } else {
            gui_interface.touch = false;
        }
    }
}