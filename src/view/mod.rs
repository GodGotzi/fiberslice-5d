/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use strum_macros::{EnumCount, EnumIter};
use three_d::{LogicalPoint, Viewport};

use crate::api::Contains;

pub mod camera;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, EnumCount, EnumIter)] //maybe performance bit worse
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

impl Contains<LogicalPoint> for Viewport {
    fn contains(&self, point: &LogicalPoint) -> bool {
        point.x > self.x as f32
            && point.x < self.x as f32 + self.width as f32
            && point.y > self.y as f32
            && point.y < self.y as f32 + self.height as f32
    }
}
