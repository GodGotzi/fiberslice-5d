use std::ops::AddAssign;

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

pub trait Flip {
    fn flip(&mut self);
}

pub trait Contains<P> {
    fn contains(&self, point: &P) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U8Color(pub [u8; 4]);

impl From<&U8Color> for [f32; 4] {
    fn from(color: &U8Color) -> Self {
        [
            color.0[0] as f32 / 255.0,
            color.0[1] as f32 / 255.0,
            color.0[2] as f32 / 255.0,
            color.0[3] as f32 / 255.0,
        ]
    }
}

impl From<U8Color> for [f32; 4] {
    fn from(color: U8Color) -> Self {
        [
            color.0[0] as f32 / 255.0,
            color.0[1] as f32 / 255.0,
            color.0[2] as f32 / 255.0,
            color.0[3] as f32 / 255.0,
        ]
    }
}

#[derive(Debug, Default)]
pub struct Average<T: std::ops::Div<f32>> {
    pub value: Option<T>,
    pub count: usize,
}

impl<T: std::ops::Div<f32, Output = T>> Average<T> {
    pub fn divide_average(self) -> Option<T> {
        if let Some(value) = self.value {
            Some(value / (self.count as f32))
        } else {
            None
        }
    }
}

impl<T: std::ops::Div<f32, Output = T> + AddAssign> AddAssign for Average<T> {
    fn add_assign(&mut self, other: Self) {
        if let Some(average) = other.divide_average() {
            self.add(average);
        }
    }
}

impl<T: std::ops::Div<f32> + AddAssign> Average<T> {
    pub fn add(&mut self, value: T) {
        if let Some(current) = self.value.as_mut() {
            *current += value;
        } else {
            self.value = Some(value);
        }

        self.count += 1;
    }
}
