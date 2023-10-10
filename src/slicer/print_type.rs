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
            Self::InternalInfill => SimpleColor::from_u8(0, 0, 255, 255),
            Self::SolidInfill => SimpleColor::from_u8(0, 255, 0, 255),
            Self::BridgeInfill => SimpleColor::from_u8(255, 0, 0, 255),
            Self::TopSolidInfill => SimpleColor::from_u8(130, 130, 0, 255),
            Self::Skirt => SimpleColor::from_u8(255, 0, 255, 255),
            Self::Brim => SimpleColor::from_u8(0, 255, 255, 255),
            Self::Support => SimpleColor::from_u8(255, 255, 255, 255),
            Self::Perimeter => SimpleColor::from_u8(255, 0, 255, 255),
            Self::WallOuter => SimpleColor::from_u8(255, 0, 0, 255),
            Self::WallInner => SimpleColor::from_u8(255, 0, 0, 255),
            Self::ExternalPerimeter => SimpleColor::from_u8(255, 255, 0, 255),
            Self::OverhangPerimeter => SimpleColor::from_u8(0, 255, 255, 255),
            Self::Unknown => SimpleColor::from_u8(0, 0, 0, 255),
        }
    }
}
