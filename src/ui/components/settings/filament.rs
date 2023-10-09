use bevy_egui::egui::{CollapsingHeader, ComboBox, DragValue};
use strum::IntoEnumIterator;

use crate::{settings::filament::FilamentType, ui::TextComponent};

impl TextComponent for crate::settings::filament::General {
    fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Extrusion Multiplier:");
            ui.add(DragValue::new(&mut self.extrusion_multiplier).max_decimals(3));
            ui.label("mm/s^2");
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Density:");
            ui.add(DragValue::new(&mut self.density).max_decimals(3));
            ui.label("g/cm^3");
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Cost:");
            ui.add(DragValue::new(&mut self.cost).max_decimals(3));
            ui.label("money/kg");
        });

        CollapsingHeader::new("Filament")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Filament Diameter:");
                    ui.add(DragValue::new(&mut self.filament.diameter).max_decimals(3));
                    ui.label("mm/s^2");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Filament Type:");
                    ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.filament.filament_type))
                        .show_ui(ui, |ui| {
                            for filament_type in FilamentType::iter() {
                                ui.selectable_value(
                                    &mut self.filament.filament_type,
                                    filament_type,
                                    format!("{:?}", filament_type),
                                );
                            }
                        });
                });
            });
    }
}

impl TextComponent for crate::settings::filament::Temperature {
    fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Nozzle Temperature:");
            ui.add(DragValue::new(&mut self.nozzle).max_decimals(3));
            ui.label("°C");
        });
        ui.horizontal(|ui| {
            crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Bed Temperature:");
            ui.add(DragValue::new(&mut self.bed).max_decimals(3));
            ui.label("°C");
        });
    }
}

impl TextComponent for crate::settings::filament::cooling::CoolingSettings {
    fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        CollapsingHeader::new("Enable")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Fan Always On:");
                    ui.checkbox(&mut self.enable.fan_always_on, "");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Enable Auto Cooling:");
                    ui.checkbox(&mut self.enable.enable_auto_cooling, "")
                });
            });

        CollapsingHeader::new("Fan")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Fan Speed:");
                    ui.add(DragValue::new(&mut self.fan.fan_speed).max_decimals(3));
                    ui.label("%");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Bridge Fan Speed:");
                    ui.add(DragValue::new(&mut self.fan.bridge_fan_speed).max_decimals(3));
                    ui.label("%");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL
                        .label(ui, "Disable Fan First Layers:");
                    ui.add(DragValue::new(&mut self.fan.disable_fan_first_layers).max_decimals(3));
                    ui.label("layers");
                });
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Full Fan at Height:");
                    ui.add(DragValue::new(&mut self.fan.full_fan_at_height).max_decimals(3));
                    ui.label("mm");
                });
            });
    }
}

impl TextComponent for crate::settings::filament::advanced::AdvancedSettings {
    fn show(&mut self, _ctx: &bevy_egui::egui::Context, ui: &mut bevy_egui::egui::Ui) {
        CollapsingHeader::new("Print Speed Override")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    crate::config::gui::settings::SETTINGS_LABEL.label(ui, "Max Volumetric Speed:");
                    ui.add(
                        DragValue::new(&mut self.print_speed_override.max_volumetric_speed)
                            .max_decimals(3),
                    );
                    ui.label("mm^3/s");
                });
            });
    }
}
