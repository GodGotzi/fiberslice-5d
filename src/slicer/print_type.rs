use strum_macros::EnumString;

use crate::api::U8Color;

#[derive(Debug, Clone, EnumString, PartialEq, Eq)]
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
    pub fn get_color(&self) -> U8Color {
        //set hard coded colors for now unique to each print type
        match &self {
            Self::InternalInfill => U8Color([0, 0, 255, 255]),
            Self::SolidInfill => U8Color([0, 255, 0, 255]),
            Self::BridgeInfill => U8Color([255, 0, 0, 255]),
            Self::TopSolidInfill => U8Color([130, 130, 0, 255]),
            Self::Skirt => U8Color([255, 0, 255, 255]),
            Self::Brim => U8Color([0, 255, 255, 255]),
            Self::Support => U8Color([255, 255, 255, 255]),
            Self::Perimeter => U8Color([255, 0, 255, 255]),
            Self::WallOuter => U8Color([255, 0, 0, 255]),
            Self::WallInner => U8Color([255, 0, 0, 255]),
            Self::ExternalPerimeter => U8Color([255, 255, 0, 255]),
            Self::OverhangPerimeter => U8Color([0, 255, 255, 255]),
            Self::Unknown => U8Color([0, 0, 0, 255]),
        }
    }
}
