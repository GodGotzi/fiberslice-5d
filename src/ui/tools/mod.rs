use egui::{Color32, ImageSource};

use crate::{GlobalState, RootEvent};

use super::{Component, UiState};

#[derive(Debug, Default)]
pub struct Tools {
    pub camera_tool: CameraToolState,
}

impl Tools {
    pub fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        CameraControlTool::with_state(&mut self.camera_tool).show(ctx, shared_state);
    }
}

pub trait ToolState {
    fn get_enabled(&mut self) -> &mut bool;

    fn get_icon(&self) -> ImageSource<'static>;
}

#[derive(Debug, Default)]
pub struct CameraToolState {
    enabled: bool,
}

impl ToolState for CameraToolState {
    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_icon(&self) -> ImageSource<'static> {
        egui::include_image!("../assets/orientation_default_30x30.png")
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
                });
        }
    }
}
