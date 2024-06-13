use strum_macros::EnumString;
use wgpu::Color;

#[derive(Debug, Clone, EnumString, PartialEq, Eq)]
pub enum PrintType {
    InternalInfill,
    SolidInfill,
    Skin,
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
            Self::InternalInfill => Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            Self::SolidInfill | Self::Skin => Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            // Srgba::new(0, 255, 0, 255),
            Self::BridgeInfill => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // Srgba::new(255, 0, 0, 255),
            Self::TopSolidInfill => Color {
                r: 130.0 / 255.0,
                g: 130.0 / 255.0,
                b: 0.0,
                a: 1.0,
            },
            // }Srgba::new(130, 130, 0, 255),
            Self::Skirt => Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            // }Srgba::new(255, 0, 255, 255),
            Self::Brim => Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            //Srgba::new(0, 255, 255, 255),
            Self::Support => Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            //}Srgba::new(255, 255, 255, 255),
            Self::Perimeter => Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            // } Srgba::new(255, 0, 255, 255),
            Self::WallOuter => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // } Srgba::new(255, 0, 0, 255),
            Self::WallInner => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // }Srgba::new(255, 0, 0, 255),
            Self::ExternalPerimeter => Color {
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            // } Srgba::new(255, 255, 0, 255),
            Self::OverhangPerimeter => Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            // } Srgba::new(0, 255, 255, 255),
            Self::Unknown => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // } Srgba::new(0, 0, 0, 255),
        }
    }
}
