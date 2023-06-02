use bevy::prelude::{Resource, ResMut, EventWriter};

use crate::view::{ViewInterface};

use self::screen::{Screen, GuiResizeEvent};

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

    pub fn ui_frame(&mut self, ctx: &bevy_egui::egui::Context, view_interface: &mut ResMut<ViewInterface>, events: &mut EventWriter<GuiResizeEvent>) {
        self.screen.ui(ctx, view_interface, events);
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

        pub fn is_touch(&self) -> bool {
            self.touch
        }
    }
    
    pub fn check_touch(
        mut contexts: EguiContexts, 
        mut gui_interface: ResMut<GuiInterface>
    ) {
        let ctx = contexts.ctx_mut();

        gui_interface.touch = ctx.is_using_pointer();
    }
}