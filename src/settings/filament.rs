use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct General {
    pub extrusion_multiplier: f32,
    pub density: f32,
    pub cost: f32,
    pub filament: FilamentSettings,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FilamentSettings {
    pub diameter: f32,
    pub filament_type: FilamentType,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Temperature {
    pub nozzle: f32,
    pub enclosure: f32,
    pub bed: f32,
}

pub mod cooling {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct CoolingSettings {
        pub enable: Enable,
        pub fan: FanSettings,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct Enable {
        pub fan_always_on: bool,
        pub enable_auto_cooling: bool,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct FanSettings {
        pub fan_speed: f32,
        pub bridge_fan_speed: f32,
        pub disable_fan_first_layers: u32,
        pub full_fan_at_height: f32,
    }
}

pub mod advanced {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct AdvancedSettings {
        pub print_speed_override: PrintSpeedOverride,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct PrintSpeedOverride {
        pub max_volumetric_speed: f32,
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum FilamentType {
    PLA,
    ABS,
}

impl Default for FilamentType {
    fn default() -> Self {
        Self::PLA
    }
}
