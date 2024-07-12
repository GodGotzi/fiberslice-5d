use egui::Color32;
use egui_code_editor::{ColorTheme, Syntax};

use crate::{viewer::GCodeSyntax, GlobalState, RootEvent};

use super::{
    widgets::reader::{EfficientReader, ReadSection},
    UiState,
};

mod debug;

pub trait Tool {
    fn show(
        &mut self,
        ctx: &egui::Context,
        shared_state: &(UiState, GlobalState<RootEvent>),
    ) -> bool;
}

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
        let mut pointer_over_tool = false;

        pointer_over_tool |=
            CameraControlTool::with_state(&mut self.camera_tool).show(ctx, shared_state);
        pointer_over_tool |= GCodeTool::with_state(&mut self.gcode_tool).show(ctx, shared_state);

        #[cfg(debug_assertions)]
        {
            pointer_over_tool |=
                Profiler::with_state(&mut self.profile_tool).show(ctx, shared_state);

            pointer_over_tool |=
                debug::DebugTool::with_state(&mut self.debug_tool).show(ctx, shared_state);
        }

        if pointer_over_tool {
            shared_state
                .0
                .pointer_in_use
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
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

impl Tool for CameraControlTool<'_> {
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

                    pointer_over_tool = ui.ui_contains_pointer();
                });
        }

        pointer_over_tool
    }
}

#[derive(Debug)]
pub struct GCodeToolState {
    enabled: bool,
    anchored: bool,
    view: ReadSection,
}

impl Default for GCodeToolState {
    fn default() -> Self {
        Self {
            enabled: false,
            anchored: false,
            view: ReadSection::new(0, 20),
        }
    }
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

impl Tool for GCodeTool<'_> {
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

            egui::Window::new("GCode")
                .open(&mut self.state.enabled)
                .movable(!self.state.anchored)
                .collapsible(false)
                .frame(frame)
                .show(ctx, |ui| {
                    if ui.button("⚓").clicked() {
                        self.state.anchored = !self.state.anchored;
                    }

                    let toolpath_server = global_state.toolpath_server.read();
                    let focused_toolpath = toolpath_server.get_focused();

                    if let Some(toolpath) = focused_toolpath {
                        let line_breaks = &toolpath.line_breaks;

                        let now = std::time::Instant::now();

                        EfficientReader::new(&mut self.state.view)
                            .id_source("code editor")
                            .with_fontsize(14.0)
                            .with_theme(ColorTheme::GRUVBOX)
                            .with_syntax(Syntax::gcode())
                            .with_numlines(true)
                            .show(ui, &toolpath.code, line_breaks);
                        println!("GCodeTool: {:?}", now.elapsed());
                    }

                    // println!("GCodeTool: {:?}", now.elapsed());
                    pointer_over_tool = ui.ui_contains_pointer();
                });
        }

        pointer_over_tool
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

impl Tool for Profiler<'_> {
    fn show(
        &mut self,
        ctx: &egui::Context,
        (_ui_state, _global_state): &(UiState, GlobalState<RootEvent>),
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

                    pointer_over_tool = ui.ui_contains_pointer();
                });
        }

        pointer_over_tool
    }
}
