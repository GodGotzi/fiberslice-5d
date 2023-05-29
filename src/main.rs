mod fiberslice;
mod component;

use std::sync::Arc;
use eframe::egui;
use egui_glow::glow::Context;
use fiberslice::screen::Screen;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("FiberSlice",
                       native_options,
                       Box::new(|cc|
                           Box::new(FiberSlice::new(cc))))
        .expect("Something went wrong while creating the frame");
}

struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {
    fn new(cc: &eframe::CreationContext) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            screen: Screen::new(cc),
        }
    }
}

impl eframe::App for FiberSlice {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.screen.ui(ctx);
    }
}