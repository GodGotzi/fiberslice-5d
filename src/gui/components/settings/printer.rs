use bevy_egui::egui::{CollapsingHeader, DragValue};

use crate::gui::{InnerTextComponent, TextComponent};

impl TextComponent for crate::settings::printer::General {
    fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Z Offset");
            ui.add(DragValue::new(&mut self.z_offset).max_decimals(3));
            ui.label("mm");
        });
    }
}

impl TextComponent for crate::settings::printer::limits::Limits {
    fn show(&mut self, ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        CollapsingHeader::new("Maximum Feedrates (mm/s)")
            .default_open(true)
            .show(ui, |ui| {
                self.max_feedrates.movements.show(
                    ctx,
                    ui,
                    "Max Feedrate".to_string(),
                    "mm/s".to_string(),
                );
            });

        CollapsingHeader::new("Maximum Accelerations (mm/s^2)")
            .default_open(true)
            .show(ui, |ui| {
                self.max_acceleration.movements.show(
                    ctx,
                    ui,
                    "Max Acceleration".to_string(),
                    "mm/s^2".to_string(),
                );

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Max Acceleration when extruding:");
                    ui.add(
                        DragValue::new(&mut self.max_acceleration.when_extruding).max_decimals(3),
                    );
                    ui.label("mm/s^2");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Max Acceleration when retracting:");
                    ui.add(
                        DragValue::new(&mut self.max_acceleration.when_retracting).max_decimals(3),
                    );
                    ui.label("mm/s^2");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Max Acceleration travel:");
                    ui.add(DragValue::new(&mut self.max_acceleration.travel).max_decimals(3));
                    ui.label("mm/s^2");
                });
            });

        CollapsingHeader::new("Jerk Limits")
            .default_open(true)
            .show(ui, |ui| {
                self.jerk_limits.movements.show(
                    ctx,
                    ui,
                    "Jerk Limit".to_string(),
                    "mm/s".to_string(),
                );
            });

        CollapsingHeader::new("Minimum Feedrates (mm/s)")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Minimum Feedrate when extruding:");
                    ui.add(
                        DragValue::new(&mut self.minimum_feedrates.when_extruding).max_decimals(3),
                    );
                    ui.label("mm/s");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Minimum Feedrate travel:");
                    ui.add(DragValue::new(&mut self.minimum_feedrates.travel).max_decimals(3));
                    ui.label("mm/s");
                });
            });
    }
}

impl TextComponent for crate::settings::printer::extruder::ExtruderSettings {
    fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        CollapsingHeader::new("Size")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Nozzle Diameter:");
                    ui.add(DragValue::new(&mut self.size.nozzle_diameter).max_decimals(1));
                    ui.label("mm");
                });
            });

        CollapsingHeader::new("Layer Height Limits(mm)")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Minimum Layer Height:");
                    ui.add(DragValue::new(&mut self.layer_height_limits.min).max_decimals(3));
                    ui.label("mm");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Maximum Layer Height:");
                    ui.add(DragValue::new(&mut self.layer_height_limits.max).max_decimals(3));
                    ui.label("mm");
                });
            });

        CollapsingHeader::new("Retraction")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Length:");
                    ui.add(DragValue::new(&mut self.retraction.length).max_decimals(3));
                    ui.label("mm");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Lift Z:");
                    ui.add(DragValue::new(&mut self.retraction.lift_z).max_decimals(3));
                    ui.label("mm");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Retract Speed:");
                    ui.add(DragValue::new(&mut self.retraction.retract_speed).max_decimals(3));
                    ui.label("mm/s");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Deretract Speed:");
                    ui.add(DragValue::new(&mut self.retraction.deretract_speed).max_decimals(3));
                    ui.label("mm/s");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Retract Restart Extra:");
                    ui.add(
                        DragValue::new(&mut self.retraction.retract_restart_extra).max_decimals(3),
                    );
                    ui.label("mm");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Minimum Travel:");
                    ui.add(DragValue::new(&mut self.retraction.minimum_travel).max_decimals(3));
                    ui.label("mm");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Retract on Layer Change:");
                    ui.checkbox(&mut self.retraction.retract_on_layer_change, "");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Wipe While Retracting:");
                    ui.checkbox(&mut self.retraction.wipe_while_retracting, "");
                });

                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Retract Amount Before Wipe:");
                    ui.add(
                        DragValue::new(&mut self.retraction.retract_amount_before_wipe)
                            .max_decimals(3),
                    );
                    ui.label("%");
                });
            });
    }
}

impl InnerTextComponent<String> for crate::settings::MovementSettings<f32> {
    fn show(
        &mut self,
        _ctx: &bevy_egui::egui::Context,
        ui: &mut bevy_egui::egui::Ui,
        prefix: String,
        suffix: String,
    ) {
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} X:", prefix));
            ui.add(DragValue::new(&mut self.x).max_decimals(3));
            ui.label(&suffix);
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} Y:", prefix));
            ui.add(DragValue::new(&mut self.y).max_decimals(3));
            ui.label(&suffix);
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} Z:", prefix));
            ui.add(DragValue::new(&mut self.z).max_decimals(3));
            ui.label(&suffix);
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} A:", prefix));
            ui.add(DragValue::new(&mut self.a).max_decimals(3));
            ui.label(&suffix);
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} B:", prefix));
            ui.add(DragValue::new(&mut self.b).max_decimals(3));
            ui.label(&suffix);
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} C:", prefix));
            ui.add(DragValue::new(&mut self.c).max_decimals(3));
            ui.label(&suffix);
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} E:", prefix));
            ui.add(DragValue::new(&mut self.e).max_decimals(3));
            ui.label(&suffix);
        });
    }
}
