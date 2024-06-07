/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use macros::NumEnum;
use strum_macros::{EnumCount, EnumIter};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, NumEnum, EnumCount, EnumIter)] //maybe performance bit worse
pub enum Orientation {
    Default,
    Diagonal,
    Top,
    Left,
    Right,
    Front,
}

#[derive(Debug, Clone, Copy, EnumCount, EnumIter)]
pub enum TransformationMode {
    Translate,
    Rotate,
    Scale,
    PlaceOnFace,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Preview,
    Prepare,
    ForceAnalytics,
}

#[allow(dead_code)]
pub enum CameraControlEvent {
    Orbit,
    TranslateTarget,
    Zoom(f32),
    TargetUpdate,
    EyeUpdate,
}
