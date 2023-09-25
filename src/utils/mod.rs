use bevy::prelude::Color;
use three_d::Srgba;
use three_d_asset::Vector3;

use crate::model::layer::WSrgba;

pub mod format;
pub mod frame;
pub mod task;

pub mod debug {
    use std::fmt::Debug;

    use super::format::PrettyFormat;

    pub struct DebugWrapper<T>(T);

    impl<T: PrettyFormat> Debug for DebugWrapper<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0.pretty_format().as_str())
        }
    }

    impl<T: PrettyFormat> From<T> for DebugWrapper<T> {
        fn from(t: T) -> Self {
            Self(t)
        }
    }
}

pub trait Contains<P> {
    fn contains(&self, point: &P) -> bool;
}

pub trait FlipYZ {
    fn flip_yz(self) -> Self;
}

impl FlipYZ for Vector3<f64> {
    fn flip_yz(self) -> Self {
        let mut s = self;

        std::mem::swap(&mut s.y, &mut s.z);
        s
    }
}

impl From<WSrgba> for Color {
    fn from(value: WSrgba) -> Self {
        Color::rgba(
            value.0.r as f32 / 255.0,
            value.0.g as f32 / 255.0,
            value.0.b as f32 / 255.0,
            1.0,
        )
    }
}