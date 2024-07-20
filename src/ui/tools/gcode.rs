use egui::{Color32, RichText};
use egui_code_editor::{ColorTheme, Syntax};

use crate::{
    ui::{
        widgets::reader::{EfficientReader, ReadSection},
        UiState,
    },
    viewer::GCodeSyntax,
    GlobalState, RootEvent,
};

use super::{create_tool, impl_tool_state_trait, impl_with_state, Tool};

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

impl_tool_state_trait!(GCodeToolState, "GCode", "📄");

create_tool!(GCodeTool, GCodeToolState);
impl_with_state!(GCodeTool, GCodeToolState);

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
