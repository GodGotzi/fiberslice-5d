use serde::{Deserialize, Serialize};

use crate::settings::MovementSettings;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Limits {
    pub max_feedrates: MaxFeedrates,
    pub max_acceleration: MaxAcceleration,
    pub jerk_limits: JerkLimits,
    pub minimum_feedrates: MinimumFeedrates,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MaxFeedrates {
    pub movements: MovementSettings<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MaxAcceleration {
    pub movements: MovementSettings<f32>,
    pub when_extruding: f32,
    pub when_retracting: f32,
    pub travel: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JerkLimits {
    pub movements: MovementSettings<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MinimumFeedrates {
    pub when_extruding: f32,
    pub travel: f32,
}
