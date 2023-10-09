pub mod filament;
pub mod printer;
pub mod ui;

use bevy::prelude::{App, Resource};
use serde::{Deserialize, Serialize};

use crate::prelude::Error;

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
pub struct SliceSettings {}

#[derive(Resource, Serialize, Deserialize)]
pub struct FilamentSettings {
    pub general: filament::General,
    pub temperature: filament::Temperature,
    pub cooling: filament::cooling::CoolingSettings,
    pub advanced: filament::advanced::AdvancedSettings,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct PrinterSettings {
    pub general: printer::General,
    pub machine_limits: printer::limits::Limits,
    pub extruder: printer::extruder::ExtruderSettings,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct SettingsPlugin;

impl bevy::app::Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrinterSettings::default())
            .insert_resource(FilamentSettings::default())
            .insert_resource(SliceSettings::default());
    }
}

impl Default for PrinterSettings {
    fn default() -> Self {
        match read_yaml("settings/printer.yaml") {
            Ok(settings) => settings,
            Err(_err) => Self {
                general: printer::General::default(),
                machine_limits: printer::limits::Limits::default(),
                extruder: printer::extruder::ExtruderSettings::default(),
            },
        }
    }
}

impl Default for FilamentSettings {
    fn default() -> Self {
        match read_yaml("settings/filament.yaml") {
            Ok(settings) => settings,
            Err(_err) => Self {
                general: filament::General::default(),
                temperature: filament::Temperature::default(),
                cooling: filament::cooling::CoolingSettings::default(),
                advanced: filament::advanced::AdvancedSettings::default(),
            },
        }
    }
}

impl Default for SliceSettings {
    fn default() -> Self {
        let content = std::fs::read_to_string("settings/slice.yaml").unwrap();
        serde_yaml::from_str::<SliceSettings>(&content).unwrap()
    }
}

fn read_yaml<T>(path: &str) -> Result<T, Error>
where
    T: Default + serde::de::DeserializeOwned,
{
    let content =
        std::fs::read_to_string(path).map_err(|err| Error::SettingsLoadError(err.to_string()))?;
    serde_yaml::from_str::<T>(&content).map_err(|err| Error::SettingsLoadError(err.to_string()))
}

impl Drop for PrinterSettings {
    fn drop(&mut self) {
        let content = serde_yaml::to_string(self).unwrap();
        std::fs::write("settings/printer.yaml", content).unwrap();
    }
}

impl Drop for FilamentSettings {
    fn drop(&mut self) {
        let content = serde_yaml::to_string(self).unwrap();
        std::fs::write("settings/filament.yaml", content).unwrap();
    }
}

impl Drop for SliceSettings {
    fn drop(&mut self) {
        let content = serde_yaml::to_string(self).unwrap();
        std::fs::write("settings/slice.yaml", content).unwrap();
    }
}
