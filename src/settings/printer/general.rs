use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GeneralSettings {
    max_height: f32,
    z_offset: f32,
}
