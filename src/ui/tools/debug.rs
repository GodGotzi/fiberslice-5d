use egui::Color32;

use crate::{
    ui::{Component, UiState},
    GlobalState, RootEvent,
};

use super::{Tool, ToolState};

#[derive(Debug, Default)]
pub struct DebugToolState {
    enabled: bool,
    anchored: bool,
}

impl ToolState for DebugToolState {
    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_popup_string(&self) -> &str {
        "Debug"
    }

    fn get_icon(&self) -> &str {
        "🐞"
    }
}

pub struct DebugTool<'a> {
    state: &'a mut DebugToolState,
}

impl<'a> DebugTool<'a> {
    pub fn with_state(state: &'a mut DebugToolState) -> Self {
        Self { state }
    }
}

impl Tool for DebugTool<'_> {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (_ui_state, global_state): &(UiState, GlobalState<RootEvent>),
    ) -> bool {
        let mut pointer_over_tool = false;

        if self.state.enabled {
            let mut frame = egui::Frame::window(&ctx.style());
            frame.fill = Color32::from_rgba_premultiplied(
                frame.fill.r(),
                frame.fill.g(),
                frame.fill.b(),
                220,
            );

            egui::Window::new("Debug")
                .open(&mut self.state.enabled)
                .movable(!self.state.anchored)
                .collapsible(false)
                .frame(frame)
                .show(ctx, |ui| {
                    if ui.button("⚓").clicked() {
                        self.state.anchored = !self.state.anchored;
                    }

                    if ui.button("🔍").clicked() {
                        global_state
                            .proxy
                            .send_event(RootEvent::RenderEvent(
                                crate::render::RenderEvent::DebugVertex,
                            ))
                            .unwrap();
                    }

                    pointer_over_tool = ui.ui_contains_pointer();
                });
        }

        pointer_over_tool
    }
}
