use three_d::LogicalPoint;
use three_d_asset::Viewport;

use crate::utils::Contains;

pub mod buffer;
pub mod camera;
pub mod environment;
pub mod visualization;

impl Contains<LogicalPoint> for Viewport {
    fn contains(&self, point: &LogicalPoint) -> bool {
        point.x > self.x as f32
            && point.x < self.x as f32 + self.width as f32
            && point.y > self.y as f32
            && point.y < self.y as f32 + self.height as f32
    }
}

#[allow(dead_code)]
pub enum Orientation {
    Default,
    Diagonal,
    Top,
    Left,
    Right,
    Front,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Preview,
    Prepare,
    ForceAnalytics,
}
