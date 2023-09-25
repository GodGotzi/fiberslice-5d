use bevy::prelude::Color;

pub mod format;

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

pub struct SimpleColor;

impl SimpleColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }
}
