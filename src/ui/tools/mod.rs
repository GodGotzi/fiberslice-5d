use egui::{Color32, RichText};
use egui_code_editor::{ColorTheme, Syntax};

use crate::{viewer::GCodeSyntax, GlobalState, RootEvent};

use super::{
    widgets::reader::{EfficientReader, ReadSection},
    UiState,
};

mod debug;
mod visibility;

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
    pub visibility_tool: visibility::VisibilityToolState,

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
        pointer_over_tool |= visibility::VisibilityTool::with_state(&mut self.visibility_tool)
            .show(ctx, shared_state);

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
                    global_state.toolpath_server.write_with_fn(|server| {
                        let focused_toolpath = server.get_focused().map(|key| key.to_string());

                        let text = if let Some(toolpath_key) = focused_toolpath.as_ref() {
                            toolpath_key.to_string()
                        } else {
                            "None".to_string()
                        };

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Selected").size(15.0));

                            egui::ComboBox::from_label("") // When created from a label the text will b shown on the side of the combobox
                                .selected_text(RichText::new(text).size(15.0)) // This is the currently selected option (in text form)
                                .show_ui(ui, |ui| {
                                    // In this closure the various options can be added

                                    let keys = server.iter_keys().cloned().collect::<Vec<_>>();

                                    for key in keys {
                                        // The first parameter is a mutable reference to allow the choice to be modified when the user selects
                                        // something else. The second parameter is the actual value of the option (to be compared with the currently)
                                        // selected one to allow egui to highlight the correct label. The third parameter is the string to show.
                                        ui.selectable_value(
                                            server.get_focused_mut(),
                                            Some(key.to_string()),
                                            key.to_string(),
                                        );
                                    }
                                });
                        });

                        if let Some(toolpath_key) = focused_toolpath {
                            let toolpath = server.get_toolpath(&toolpath_key).unwrap();
                            let line_breaks = &toolpath.line_breaks;

                            EfficientReader::new(&mut self.state.view)
                                .id_source("code editor")
                                .with_fontsize(14.0)
                                .with_theme(ColorTheme::GRUVBOX)
                                .with_syntax(Syntax::gcode())
                                .with_numlines(true)
                                // .with_focus(Some(ReadSection::new(0, 20)))
                                .show(ui, &toolpath.code, line_breaks);
                        }
                    });

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
                    puffin_egui::profiler_ui(ui);

                    pointer_over_tool = ui.ui_contains_pointer();
                });
        }

        pointer_over_tool
    }
}
