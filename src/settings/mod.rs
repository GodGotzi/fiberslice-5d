pub mod filament;
pub mod printer;

use bevy::prelude::{App, Resource};
use serde::{Deserialize, Serialize};

pub use crate::slicer::settings as slicer;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MovementSettings<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub a: T,
    pub b: T,
    pub c: T,
    pub e: T,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct PrinterSettings {
    pub general: printer::general::GeneralSettings,
    pub machine_limits: printer::limits::Limits,
    pub extruder: printer::extruder::ExtruderSettings,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct FilamentSettings {}

#[derive(Resource, Serialize, Deserialize)]
pub struct SettingsPlugin;

impl bevy::app::Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrinterSettings::default())
            .insert_resource(FilamentSettings::default())
            .insert_resource(slicer::SliceSettings::default());
    }
}

impl Default for PrinterSettings {
    fn default() -> Self {
        let content = std::fs::read_to_string("settings/printer.yaml").unwrap();
        match serde_yaml::from_str::<PrinterSettings>(&content) {
            Ok(settings) => settings,
            Err(err) => {
                println!("Error: {}", err);
                Self {
                    general: printer::general::GeneralSettings::default(),
                    machine_limits: printer::limits::Limits::default(),
                    extruder: printer::extruder::ExtruderSettings::default(),
                }
            }
        }
    }
}

impl Default for FilamentSettings {
    fn default() -> Self {
        //let content = std::fs::read_to_string("settings/filament.yaml").unwrap();
        //serde_yaml::from_str::<FilamentSettings>(&content).unwrap()
        Self {}
    }
}

impl Default for slicer::SliceSettings {
    fn default() -> Self {
        //let content = std::fs::read_to_string("settings/slicer.yaml").unwrap();
        //serde_yaml::from_str::<slicer::SliceSettings>(&content).unwrap()
        Self {}
    }
}
