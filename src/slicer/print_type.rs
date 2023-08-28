use strum_macros::EnumString;
use three_d_asset::Srgba;

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
    pub fn get_color(&self) -> Srgba {
        //set hard coded colors for now unique to each print type
        match &self {
            Self::InternalInfill => Srgba::new(0, 0, 255, 255),
            Self::SolidInfill => Srgba::new(0, 255, 0, 255),
            Self::BridgeInfill => Srgba::new(255, 0, 0, 255),
            Self::TopSolidInfill => Srgba::new(130, 130, 0, 255),
            Self::Skirt => Srgba::new(255, 0, 255, 255),
            Self::Brim => Srgba::new(0, 255, 255, 255),
            Self::Support => Srgba::new(255, 255, 255, 255),
            Self::Perimeter => Srgba::new(255, 0, 255, 255),
            Self::WallOuter => Srgba::new(255, 0, 0, 255),
            Self::WallInner => Srgba::new(255, 0, 0, 255),
            Self::ExternalPerimeter => Srgba::new(255, 255, 0, 255),
            Self::OverhangPerimeter => Srgba::new(0, 255, 255, 255),
            Self::Unknown => Srgba::new(0, 0, 0, 255),
        }
    }
}
