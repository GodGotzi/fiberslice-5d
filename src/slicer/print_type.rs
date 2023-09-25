use bevy::prelude::Color;
use strum_macros::EnumString;

use crate::utils::SimpleColor;

#[derive(Debug, Clone, EnumString)]
pub enum PrintType {
    InternalInfill,
    SolidInfill,
    BridgeInfill,
    TopSolidInfill,
    Skirt,
    Brim,
    Support,
    Perimeter,
    ExternalPerimeter,
    OverhangPerimeter,
    WallOuter,
    WallInner,
    Unknown,
}

impl PrintType {
    pub fn get_color(&self) -> Color {
        //set hard coded colors for now unique to each print type
        match &self {
            Self::InternalInfill => SimpleColor::new(0, 0, 255, 255),
            Self::SolidInfill => SimpleColor::new(0, 255, 0, 255),
            Self::BridgeInfill => SimpleColor::new(255, 0, 0, 255),
            Self::TopSolidInfill => SimpleColor::new(130, 130, 0, 255),
            Self::Skirt => SimpleColor::new(255, 0, 255, 255),
            Self::Brim => SimpleColor::new(0, 255, 255, 255),
            Self::Support => SimpleColor::new(255, 255, 255, 255),
            Self::Perimeter => SimpleColor::new(255, 0, 255, 255),
            Self::WallOuter => SimpleColor::new(255, 0, 0, 255),
            Self::WallInner => SimpleColor::new(255, 0, 0, 255),
            Self::ExternalPerimeter => SimpleColor::new(255, 255, 0, 255),
            Self::OverhangPerimeter => SimpleColor::new(0, 255, 255, 255),
            Self::Unknown => SimpleColor::new(0, 0, 0, 255),
        }
    }
}
