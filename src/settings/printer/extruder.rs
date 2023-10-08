use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExtruderSettings {
    pub size: Size,
    pub layer_height_limits: LayerHeightLimits,
    pub retraction: Retraction,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Size {
    pub nozzle_diameter: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerHeightLimits {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Retraction {
    pub length: f32,
    pub lift_z: f32,
    pub retract_speed: f32,
    pub deretract_speed: f32,
    pub retract_restart_extra: f32,
    pub minimum_travel: f32,
    pub retract_on_layer_change: bool,
    pub wipe_while_retracting: bool,
    pub retract_amount_before_wipe: f32,
}
