use egui::{Button, Color32, RichText};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

use crate::config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
pub enum GizmoTool {
    Translate,
    Rotate,
    Scale,
    AlignPlane,
}

const GIZMO_TOOL_ICONS: [(&str, char); GizmoTool::COUNT] = [
    ("Translate", 'T'),
    ("Rotate", 'R'),
    ("Scale", 'S'),
    ("Align Plane", 'A'),
];

pub struct GizmoTools {
    selected: Option<GizmoTool>,
}

impl Default for GizmoTools {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl GizmoTools {
    pub fn show_icons(
        &mut self,
        ui: &mut egui::Ui,
        shared_state: &(crate::ui::UiState, crate::GlobalState<crate::RootEvent>),
    ) {
        for (tool, (name, icon)) in GizmoTool::iter().zip(GIZMO_TOOL_ICONS.iter()) {
            let is_selected = self.selected == Some(tool);
            let button = config::gui::TOOL_TOGGLE_BUTTON;

            let image_button = Button::new(RichText::new(*icon).size(35.0))
                .frame(true)
                .selected(is_selected)
                .rounding(5.0);

            ui.allocate_ui(
                [button.size.0 + button.border, button.size.1 + button.border].into(),
                |ui| {
                    let response = ui.add_sized([button.size.0, button.size.1], image_button);

                    if response.clicked() {
                        self.selected = Some(tool);
                    } else if response.hovered() {
                        egui::popup::show_tooltip(
                            ui.ctx(),
                            ui.layer_id(),
                            egui::Id::new(format!("popup-{}", name)),
                            |ui| {
                                ui.label(name.to_string());
                            },
                        );
                    }
                },
            );

            ui.add_space(5.0);
        }
    }

    pub fn show_tool_wíndow(
        &mut self,
        ui: &mut egui::Ui,
        shared_state: &(crate::ui::UiState, crate::GlobalState<crate::RootEvent>),
    ) {
        let index = self.selected.as_ref().map(|tool| *tool as usize);

        if let Some(index) = index {
            let (name, _) = GIZMO_TOOL_ICONS[index];

            let mut frame = egui::Frame::window(ui.style());
            frame.fill = Color32::from_rgba_premultiplied(
                frame.fill.r(),
                frame.fill.g(),
                frame.fill.b(),
                220,
            );

            egui::Window::new(name)
                .open(&mut true)
                .movable(true)
                .collapsible(false)
                .resizable(false)
                .frame(frame)
                .show(ui.ctx(), |ui| {
                    ui.separator();
                });
        }
    }
}
