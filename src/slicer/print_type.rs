use glam::Vec4;
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};
use wgpu::Color;

#[derive(Debug, Clone, EnumString, EnumCount, IntoStaticStr, EnumIter, PartialEq, Eq)]
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

impl From<PrintType> for Color {
    fn from(print_type: PrintType) -> Self {
        match print_type {
            PrintType::InternalInfill => Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            PrintType::SolidInfill | PrintType::Skin => Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            // Srgba::new(0, 255, 0, 255),
            PrintType::BridgeInfill => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // Srgba::new(255, 0, 0, 255),
            PrintType::TopSolidInfill => Color {
                r: 130.0 / 255.0,
                g: 130.0 / 255.0,
                b: 0.0,
                a: 1.0,
            },
            // }Srgba::new(130, 130, 0, 255),
            PrintType::Skirt => Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            // }Srgba::new(255, 0, 255, 255),
            PrintType::Brim => Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            //Srgba::new(0, 255, 255, 255),
            PrintType::Support => Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            //}Srgba::new(255, 255, 255, 255),
            PrintType::Perimeter => Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            // } Srgba::new(255, 0, 255, 255),
            PrintType::WallOuter => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // } Srgba::new(255, 0, 0, 255),
            PrintType::WallInner => Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            // }Srgba::new(255, 0, 0, 255),
            PrintType::ExternalPerimeter => Color {
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            // } Srgba::new(255, 255, 0, 255),
            PrintType::OverhangPerimeter => Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            // } Srgba::new(0, 255, 255, 255),
            PrintType::Unknown => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

impl From<&PrintType> for Color {
    fn from(print_type: &PrintType) -> Self {
        print_type.clone().into()
    }
}

impl From<PrintType> for Vec4 {
    fn from(print_type: PrintType) -> Self {
        let color: Color = print_type.into();
        Vec4::new(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        )
    }
}

impl From<&PrintType> for Vec4 {
    fn from(print_type: &PrintType) -> Self {
        let color: Color = print_type.clone().into();
        Vec4::new(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        )
    }
}
