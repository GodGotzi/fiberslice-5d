use glam::{vec3, Vec3, Vec4};
use slicer::Settings;
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};
use wgpu::Color;

use crate::{GlobalState, RootEvent};

#[derive(Debug, Default)]
pub struct Slicer {
    pub settings: Settings,
}

impl Slicer {
    pub fn slice(&self, global_state: &GlobalState<RootEvent>) -> Result<(), String> {
        let model_server_read = global_state.viewer.model_server.read();

        let mut models: Vec<(Vec<Vec3>, Vec<slicer::IndexedTriangle>)> = model_server_read
            .iter_entries()
            .map(|entry| entry.1)
            .collect();

        for (vertices, _) in models.iter_mut() {
            let (min, max) = vertices.iter().fold(
                (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
                |(min, max), v| (min.min(*v), max.max(*v)),
            );

            let transform = glam::Mat4::from_translation(vec3(
                (210.0 - (max.x + min.x)) / 2.,
                (210.0 - (max.y + min.y)) / 2.,
                -min.z,
            ));

            for v in vertices.iter_mut() {
                *v = (transform * v.extend(1.0)).truncate();
            }
        }

        let models: Vec<(Vec<slicer::Vertex>, Vec<slicer::IndexedTriangle>)> = models
            .into_iter()
            .map(|(vertices, faces)| {
                (
                    vertices
                        .into_iter()
                        .map(|v| slicer::Vertex {
                            x: v.x as f64,
                            y: v.y as f64,
                            z: v.z as f64,
                        })
                        .collect::<Vec<slicer::Vertex>>(),
                    faces,
                )
            })
            .collect();

        let settings = self.settings.clone();

        let result = slicer::slice(&models, &settings).expect("Failed to slice model");

        println!("Sliced model {:?}", result);

        Ok(())
    }
}

#[derive(Debug, Clone, EnumString, EnumCount, IntoStaticStr, EnumIter, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathType {
    Work { print_type: PrintType, travel: bool },
    Setup,
}

impl core::hash::Hash for PathType {
    fn hash<H: core::hash::Hasher>(&self, ra_expand_state: &mut H) {
        core::mem::discriminant(self).hash(ra_expand_state);
        match self {
            PathType::Work { print_type, travel } => {
                if *travel {
                    true.hash(ra_expand_state);
                    // PrintType::Unknown.hash(ra_expand_state);
                } else {
                    false.hash(ra_expand_state);
                    print_type.hash(ra_expand_state);
                }
            }
            PathType::Setup => {}
        }
    }
}

impl From<&PathType> for &str {
    fn from(print_type: &PathType) -> Self {
        match print_type {
            PathType::Work { print_type, travel } => {
                if *travel {
                    "Travel"
                } else {
                    print_type.into()
                }
            }
            PathType::Setup => "Setup",
        }
    }
}

impl From<PathType> for Color {
    fn from(print_type: PathType) -> Self {
        match print_type {
            PathType::Work { print_type, travel } => {
                if travel {
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }
                } else {
                    print_type.into()
                }
            }
            PathType::Setup => Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

impl From<&PathType> for Color {
    fn from(print_type: &PathType) -> Self {
        print_type.clone().into()
    }
}

impl From<PathType> for Vec4 {
    fn from(print_type: PathType) -> Self {
        let color: Color = print_type.into();
        Vec4::new(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        )
    }
}

impl From<&PathType> for Vec4 {
    fn from(print_type: &PathType) -> Self {
        let color: Color = print_type.clone().into();
        Vec4::new(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        )
    }
}

impl PathType {
    pub fn print_type(&self) -> Option<PrintType> {
        match self {
            PathType::Work {
                print_type,
                travel: _,
            } => Some(print_type.clone()),
            PathType::Setup => None,
        }
    }

    pub fn update_type(&mut self, print_type: PrintType) {
        match self {
            PathType::Work {
                print_type: current_type,
                ..
            } => {
                *current_type = print_type;
            }
            PathType::Setup => {
                *self = PathType::Work {
                    print_type,
                    travel: false,
                };
            }
        }
    }

    pub fn set_travel(&mut self, travel: bool) {
        match self {
            PathType::Work {
                print_type: _,
                travel: current_travel,
            } => {
                *current_travel = travel;
            }
            PathType::Setup => {}
        }
    }

    pub fn is_travel(&self) -> bool {
        match self {
            PathType::Work { travel, .. } => *travel,
            PathType::Setup => false,
        }
    }

    /// Returns the bit representation of the path type.
    /// The first bit is the setup flag, the second bit is the travel flag. The rest of the bits are the print type.
    /// The print type is represented by the enum variant index.
    /// # Example
    /// ```
    /// use slicer::print_type::{PathType, PrintType};
    ///
    /// let path_type = PathType::Work {
    ///
    ///    print_type: PrintType::InternalInfill,
    ///   travel: false,
    /// };
    ///
    /// assert_eq!(path_type.bit_representation(), 1);
    ///
    pub fn bit_representation(&self) -> u32 {
        match self {
            PathType::Work { print_type, travel } => {
                if *travel {
                    0x02
                } else {
                    0x01 << ((print_type.clone() as u32) + 0x02)
                }
            }
            PathType::Setup => 0x01,
        }
    }
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
