use std::fs;
use strum_macros::Display;
use three_d::*;

use serde::{Deserialize, Serialize};

use crate::{math::VirtualPlane, prelude::Error};

#[derive(Debug, Clone, Copy, Display)]
pub enum Setup {
    Anycubic,
    FiberPrinter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupContext {
    box_offset: Vector3<f32>,
    print_box: PrintBox,
    printer_glb_path: String,
}

impl TryFrom<Setup> for SetupContext {
    type Error = crate::error::Error;

    fn try_from(setup: Setup) -> Result<Self, Error> {
        let path = format!("setup/{}.yaml", setup);
        let content = fs::read_to_string(path)?;
        let config: SetupContext = serde_yaml::from_str(&content)
            .map_err(|_| crate::error::Error::SetupError("Could not parse config file".into()))?;

        Ok(config)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintBox {
    plane: VirtualPlane,
    height: f32,
}
