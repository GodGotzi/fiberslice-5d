use egui::{Color32, FontId, RichText};
use slicer::MovePrintType;
use strum::EnumCount;
use wgpu::Color;

use crate::{ui::UiState, GlobalState, RootEvent};

use super::{Tool, ToolState};

#[derive(Debug)]
pub struct VisibilityToolState {
    enabled: bool,
    anchored: bool,
    print_types: [bool; MovePrintType::COUNT],
    travel: bool,
    setup: bool,
}

impl Default for VisibilityToolState {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            anchored: Default::default(),
            print_types: [true; MovePrintType::COUNT],
            travel: false,
            setup: false,
        }
    }
}

impl ToolState for VisibilityToolState {
    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_popup_string(&self) -> &str {
        "Visibility"
    }

    fn get_icon(&self) -> &str {
        "👓"
    }
}

pub struct VisibilityTool<'a> {
    state: &'a mut VisibilityToolState,
}

impl<'a> VisibilityTool<'a> {
    pub fn with_state(state: &'a mut VisibilityToolState) -> Self {
        Self { state }
    }
}

impl Tool for VisibilityTool<'_> {
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

            egui::Window::new("Visibility")
                .open(&mut self.state.enabled)
                .movable(!self.state.anchored)
                .collapsible(false)
                .resizable(false)
                .frame(frame)
                .show(ctx, |ui| {
                    ui.separator();

                    let old_print_types = self.state.print_types;
                    let old_travel = self.state.travel;
                    let old_setup = self.state.setup;

                    if let Some(count_map) = global_state
                        .viewer
                        .toolpath_server
                        .read()
                        .get_toolpath()
                        .map(|toolpath| &toolpath.count_map)
                    {
                        egui::CollapsingHeader::new("Print Types")
                            .default_open(true)
                            .show(ui, |ui| {
                                for (print_type, count) in count_map.iter() {
                                    let str_type: String = format!("{}", print_type);
                                    let color_vec = print_type.into_color_vec4();

                                    let color: wgpu::Color = Color {
                                        r: color_vec.x as f64,
                                        g: color_vec.y as f64,
                                        b: color_vec.z as f64,
                                        a: color_vec.w as f64,
                                    };

                                    let egui_color = Color32::from_rgba_premultiplied(
                                        (color.r * 255.0) as u8,
                                        (color.g * 255.0) as u8,
                                        (color.b * 255.0) as u8,
                                        (color.a * 255.0) as u8,
                                    );

                                    ui.horizontal(|ui| {
                                        ui.checkbox(
                                            &mut self.state.print_types[*print_type as usize],
                                            RichText::new(str_type)
                                                .font(FontId::monospace(15.0))
                                                .strong()
                                                .color(egui_color),
                                        );

                                        ui.add_space(25.0);

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(format!("{:7}", count))
                                                        .font(FontId::monospace(15.0))
                                                        .strong(),
                                                );
                                            },
                                        );
                                    });
                                }

                                ui.separator();
                            });

                        ui.horizontal(|ui| {
                            ui.checkbox(
                                &mut self.state.travel,
                                RichText::new("Travel")
                                    .font(FontId::monospace(15.0))
                                    .strong()
                                    .color(Color32::BLACK),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(
                                &mut self.state.setup,
                                RichText::new("Setup")
                                    .font(FontId::monospace(15.0))
                                    .strong()
                                    .color(Color32::BLACK),
                            );
                        });
                    }

                    if old_print_types != self.state.print_types
                        || old_travel != self.state.travel
                        || old_setup != self.state.setup
                    {
                        let mut visibility = 0;

                        for (index, visible) in self.state.print_types.iter().enumerate() {
                            if *visible {
                                visibility |= 1 << index;
                            }
                        }

                        visibility <<= 2;

                        visibility |= if self.state.travel { 0x02 } else { 0 };

                        visibility |= if self.state.setup { 0x01 } else { 0 };

                        global_state
                            .viewer
                            .toolpath_server
                            .write()
                            .set_visibility(visibility);
                    }

                    ui.separator();

                    pointer_over_tool = ui.ui_contains_pointer();
                });
        }

        pointer_over_tool
    }
}
