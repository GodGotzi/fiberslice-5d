pub mod settingsbar;

pub mod printer {
    use bevy_egui::egui::DragValue;

    use crate::gui::{InnerTextComponent, TextComponent};

    impl TextComponent for crate::settings::printer::general::GeneralSettings {
        fn show(&mut self, _ctx: &bevy_egui::egui::Context, _ui: &mut bevy_egui::egui::Ui) {}
    }

    impl TextComponent for crate::settings::printer::limits::Limits {
        fn show(&mut self, ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
            ui.label("Maximum Feedrates (mm/s)");
            ui.separator();

            self.max_feedrates
                .movements
                .show(ctx, ui, "Max Feedrate".to_string());

            ui.separator();

            ui.label("Maximum Accelerations (mm/s^2)");
            ui.separator();

            self.max_acceleration
                .movements
                .show(ctx, ui, "Max Acceleration".to_string());

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL
                    .label(ui, "Max Acceleration when extruding:");
                ui.add(DragValue::new(&mut self.max_acceleration.when_extruding).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL
                    .label(ui, "Max Acceleration when retracting:");
                ui.add(
                    DragValue::new(&mut self.max_acceleration.when_retracting).fixed_decimals(3),
                );
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Max Acceleration travel:");
                ui.add(DragValue::new(&mut self.max_acceleration.travel).fixed_decimals(3));
            });

            ui.separator();

            ui.label("Jerk Limits (mm/s)");
            ui.separator();

            self.jerk_limits
                .movements
                .show(ctx, ui, "Jerk Limit".to_string());

            ui.separator();

            ui.label("Minimum Feedrates (mm/s)");
            ui.separator();

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL
                    .label(ui, "Minimum Feedrate when extruding:");
                ui.add(
                    DragValue::new(&mut self.minimum_feedrates.when_extruding).fixed_decimals(3),
                );
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Minimum Feedrate travel:");
                ui.add(DragValue::new(&mut self.minimum_feedrates.travel).fixed_decimals(3));
            });
        }
    }

    impl TextComponent for crate::settings::printer::extruder::ExtruderSettings {
        fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
            ui.label("Size");
            ui.separator();

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Nozzle Diameter:");
                ui.add(DragValue::new(&mut self.size.nozzle_diameter).fixed_decimals(1));
                ui.label("mm");
            });
            ui.separator();

            ui.label("Layer Height Limits(mm)");
            ui.separator();

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Minimum Layer Height:");
                ui.add(DragValue::new(&mut self.layer_height_limits.min).fixed_decimals(3));
                ui.label("mm");
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Maximum Layer Height:");
                ui.add(DragValue::new(&mut self.layer_height_limits.max).fixed_decimals(3));
                ui.label("mm");
            });

            ui.separator();

            ui.label("Retraction");
            ui.separator();

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Length:");
                ui.add(DragValue::new(&mut self.retraction.length).fixed_decimals(3));
                ui.label("mm");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Lift Z:");
                ui.add(DragValue::new(&mut self.retraction.lift_z).fixed_decimals(3));
                ui.label("mm");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Retract Speed:");
                ui.add(DragValue::new(&mut self.retraction.retract_speed).fixed_decimals(3));
                ui.label("mm/s");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Deretract Speed:");
                ui.add(DragValue::new(&mut self.retraction.deretract_speed).fixed_decimals(3));
                ui.label("mm/s");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Retract Restart Extra:");
                ui.add(
                    DragValue::new(&mut self.retraction.retract_restart_extra).fixed_decimals(3),
                );
                ui.label("mm");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Minimum Travel:");
                ui.add(DragValue::new(&mut self.retraction.minimum_travel).fixed_decimals(3));
                ui.label("mm");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Retract on Layer Change:");
                ui.checkbox(&mut self.retraction.retract_on_layer_change, "");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Wipe While Retracting:");
                ui.checkbox(&mut self.retraction.wipe_while_retracting, "");
            });

            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL
                    .label(ui, "Retract Amount Before Wipe:");
                ui.add(
                    DragValue::new(&mut self.retraction.retract_amount_before_wipe)
                        .fixed_decimals(3),
                );
                ui.label("mm");
            });

            ui.separator();
        }
    }

    impl InnerTextComponent<String> for crate::settings::MovementSettings<f32> {
        fn show(
            &mut self,
            _ctx: &bevy_egui::egui::Context,
            ui: &mut bevy_egui::egui::Ui,
            prefix: String,
        ) {
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} X:", prefix));
                ui.add(DragValue::new(&mut self.x).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} Y:", prefix));
                ui.add(DragValue::new(&mut self.y).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} Z:", prefix));
                ui.add(DragValue::new(&mut self.z).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} A:", prefix));
                ui.add(DragValue::new(&mut self.a).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} B:", prefix));
                ui.add(DragValue::new(&mut self.b).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} C:", prefix));
                ui.add(DragValue::new(&mut self.c).fixed_decimals(3));
            });
            ui.horizontal(|ui| {
                crate::config::gui::settings::SETTINGS_LABEL.label(ui, format!("{} E:", prefix));
                ui.add(DragValue::new(&mut self.e).fixed_decimals(3));
            });
        }
    }
}
