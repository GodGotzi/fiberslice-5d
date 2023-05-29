mod fiberslice;
mod component;

use fiberslice::screen::Screen;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

fn main() {
    let mut fiberslice = FiberSlice::new();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(move |contexts: EguiContexts| {
            fiberslice.show_ui(contexts)
        })
        .run();
}

struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {
    fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }
}

impl FiberSlice {
    fn show_ui(&mut self, mut contexts: EguiContexts) {
        let ctx = contexts.ctx_mut();

        self.screen.ui(ctx);
    }
}