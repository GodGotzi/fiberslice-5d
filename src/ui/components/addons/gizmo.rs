use egui::{Color32, ImageButton, Visuals};
use egui_extras::Size;
use egui_grid::GridBuilder;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

use crate::{config, ui::icon::get_gizmo_tool_icon};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
pub enum GizmoTool {
    Translate,
    Rotate,
    Scale,
    Flatten,
}

const GIZMO_TOOL_LABELS: [&str; GizmoTool::COUNT] =
    [("Translate"), ("Rotate"), ("Scale"), ("Flatten")];

#[derive(Debug, Default)]
pub struct GizmoTools {
    selected: Option<GizmoTool>,
}

impl GizmoTools {
    pub fn show_icons(
        &mut self,
        ui: &mut egui::Ui,
        _shared_state: &(crate::ui::UiState, crate::GlobalState<crate::RootEvent>),
    ) {
        ui.vertical(|ui| {
            let mut builder = GridBuilder::new();

            let button = config::gui::GIZMO_TOGGLE_BUTTON;

            for _ in 0..GizmoTool::COUNT {
                builder = builder.new_row(Size::remainder());
                builder = builder.new_row_align(Size::exact(button.size.0), egui::Align::Center);
                builder = builder.cell(Size::exact(button.size.0));
            }

            builder = builder.new_row(Size::remainder());

            *ui.visuals_mut() = Visuals::light();
            ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

            builder.show(ui, |mut grid| {
                for (tool, name) in GizmoTool::iter().zip(GIZMO_TOOL_LABELS.iter()) {
                    grid.cell(|ui| {
                        // let is_selected = self.selected == Some(tool);

                        let image_button = ImageButton::new(get_gizmo_tool_icon(tool)).frame(true);

                        let response = ui.add(image_button);

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
                    });
                }
            });
        });
    }

    pub fn show_tool_wíndow(
        &mut self,
        ui: &mut egui::Ui,
        _shared_state: &(crate::ui::UiState, crate::GlobalState<crate::RootEvent>),
    ) {
        let index = self.selected.as_ref().map(|tool| *tool as usize);

        if let Some(index) = index {
            let name = GIZMO_TOOL_LABELS[index];

            let mut frame = egui::Frame::window(ui.style());
            frame.fill = Color32::from_rgba_premultiplied(
                frame.fill.r(),
                frame.fill.g(),
                frame.fill.b(),
                220,
            );

            let mut open = true;

            egui::Window::new(name)
                .open(&mut open)
                .movable(true)
                .collapsible(false)
                .resizable(false)
                .frame(frame)
                .show(ui.ctx(), |ui| {
                    ui.separator();
                });

            if !open {
                self.selected = None;
            }
        }
    }
}
