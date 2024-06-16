use egui::Color32;

use crate::{GlobalState, RootEvent};

use super::{Component, UiState};

mod debug;

#[derive(Debug, Default)]
pub struct Tools {
    pub camera_tool: CameraToolState,
    pub gcode_tool: GCodeToolState,

    #[cfg(debug_assertions)]
    pub profile_tool: ProfilerState,

    #[cfg(debug_assertions)]
    pub debug_tool: debug::DebugToolState,
}

impl Tools {
    pub fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        CameraControlTool::with_state(&mut self.camera_tool).show(ctx, shared_state);
        GCodeTool::with_state(&mut self.gcode_tool).show(ctx, shared_state);

        #[cfg(debug_assertions)]
        Profiler::with_state(&mut self.profile_tool).show(ctx, shared_state);

        #[cfg(debug_assertions)]
        debug::DebugTool::with_state(&mut self.debug_tool).show(ctx, shared_state);
    }
}

pub trait ToolState {
    fn get_enabled(&mut self) -> &mut bool;

    fn get_popup_string(&self) -> &str {
        ""
    }

    fn get_icon(&self) -> &str;
}

#[derive(Debug, Default)]
pub struct CameraToolState {
    enabled: bool,
    anchored: bool,
}

impl ToolState for CameraToolState {
    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_popup_string(&self) -> &str {
        "Camera"
    }

    fn get_icon(&self) -> &str {
        "📷"
    }
}

#[derive(Debug)]
pub struct CameraControlTool<'a> {
    state: &'a mut CameraToolState,
}

impl<'a> CameraControlTool<'a> {
    pub fn with_state(state: &'a mut CameraToolState) -> Self {
        Self { state }
    }
}

impl Component for CameraControlTool<'_> {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (_ui_state, global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        if self.state.enabled {
            let mut frame = egui::Frame::window(&ctx.style());
            frame.fill = Color32::from_rgba_premultiplied(
                frame.fill.r(),
                frame.fill.g(),
                frame.fill.b(),
                220,
            );

            egui::Window::new("Camera Controls")
                .open(&mut self.state.enabled)
                .collapsible(false)
                .movable(!self.state.anchored)
                .frame(frame)
                .show(ctx, |ui| {
                    if ui.button("⚓").clicked() {
                        self.state.anchored = !self.state.anchored;
                    }

                    global_state.camera_controller.write_with_fn(|controller| {
                        ui.horizontal(|ui| {
                            ui.label(format!("{:20}", "Rotate Speed"));

                            let slider =
                                egui::Slider::new(&mut controller.rotate_speed, -0.1..=0.1);
                            ui.add(slider);

                            if ui.button("Reset").clicked() {
                                controller.rotate_speed = 0.0;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!("{:20}", "Zoom Speed  "));

                            let slider = egui::Slider::new(&mut controller.zoom_speed, -5.0..=5.0);
                            ui.add(slider);

                            if ui.button("Reset").clicked() {
                                controller.zoom_speed = 0.0;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!("{:20}", "Move Speed  "));

                            let slider = egui::Slider::new(&mut controller.move_speed, -1.0..=1.0);
                            ui.add(slider);

                            if ui.button("Reset").clicked() {
                                controller.move_speed = 0.0;
                            }
                        });
                    });
                });
        }
    }
}

#[derive(Debug, Default)]
pub struct GCodeToolState {
    enabled: bool,
    anchored: bool,
}

impl ToolState for GCodeToolState {
    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_popup_string(&self) -> &str {
        "GCode"
    }

    fn get_icon(&self) -> &str {
        "📄"
    }
}

pub struct GCodeTool<'a> {
    state: &'a mut GCodeToolState,
}

impl<'a> GCodeTool<'a> {
    pub fn with_state(state: &'a mut GCodeToolState) -> Self {
        Self { state }
    }
}

impl Component for GCodeTool<'_> {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (_ui_state, global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        if self.state.enabled {
            let mut frame = egui::Frame::window(&ctx.style());
            frame.fill = Color32::from_rgba_premultiplied(
                frame.fill.r(),
                frame.fill.g(),
                frame.fill.b(),
                220,
            );

            egui::Window::new("GCode")
                .open(&mut self.state.enabled)
                .movable(!self.state.anchored)
                .collapsible(false)
                .frame(frame)
                .show(ctx, |ui| {
                    if ui.button("⚓").clicked() {
                        self.state.anchored = !self.state.anchored;
                    }

                    ui.label("GCode");
                });
        }
    }
}

#[derive(Debug, Default)]
pub struct ProfilerState {
    enabled: bool,
    anchored: bool,
}

impl ToolState for ProfilerState {
    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_popup_string(&self) -> &str {
        "Profile"
    }

    fn get_icon(&self) -> &str {
        "📊"
    }
}

pub struct Profiler<'a> {
    state: &'a mut ProfilerState,
}

impl<'a> Profiler<'a> {
    pub fn with_state(state: &'a mut ProfilerState) -> Self {
        Self { state }
    }
}

impl Component for Profiler<'_> {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (_ui_state, global_state): &(UiState, GlobalState<RootEvent>),
    ) {
        if self.state.enabled {
            let mut frame = egui::Frame::window(&ctx.style());
            frame.fill = Color32::from_rgba_premultiplied(
                frame.fill.r(),
                frame.fill.g(),
                frame.fill.b(),
                220,
            );

            egui::Window::new("Profile")
                .open(&mut self.state.enabled)
                .movable(!self.state.anchored)
                .collapsible(false)
                .frame(frame)
                .show(ctx, |ui| {
                    if ui.button("⚓").clicked() {
                        self.state.anchored = !self.state.anchored;
                    }

                    puffin_egui::profiler_ui(ui);
                });
        }
    }
}
