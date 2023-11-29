use three_d::Srgba;

pub mod gcode;
pub mod mesh;
pub mod shapes;

pub struct ToolPath;

pub trait SrgbaToArray<T> {
    fn to_array(&self) -> [T; 4];
}

impl SrgbaToArray<u8> for Srgba {
    fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub trait SrgbaArrayToFloat {
    fn to_float(&self) -> [f32; 4];
}

impl SrgbaArrayToFloat for [u8; 4] {
    fn to_float(&self) -> [f32; 4] {
        [
            self[0] as f32 / 255.0,
            self[1] as f32 / 255.0,
            self[2] as f32 / 255.0,
            self[3] as f32 / 255.0,
        ]
    }
}
