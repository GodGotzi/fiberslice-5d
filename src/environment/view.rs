/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

use macros::NumEnum;
use strum_macros::{EnumCount, EnumIter};
use three_d::{LogicalPoint, Viewport};

use three_d::{vec3, Camera, Deg, Vec3};

use crate::api::Contains;

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

impl Contains<LogicalPoint> for Viewport {
    fn contains(&self, point: &LogicalPoint) -> bool {
        point.x > self.x as f32
            && point.x < self.x as f32 + self.width as f32
            && point.y > self.y as f32
            && point.y < self.y as f32 + self.height as f32
    }
}

#[allow(dead_code)]
pub enum CameraControlEvent {
    Orbit,
    TranslateTarget,
    Zoom(f32),
    TargetUpdate,
    EyeUpdate,
}

pub struct CameraBuilder {
    viewport: Option<Viewport>,
    position: Option<Vec3>,
    target: Option<Vec3>,
    up: Option<Vec3>,
    fov: Option<Deg<f32>>,
    near: Option<f32>,
    far: Option<f32>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            viewport: None,
            position: None,
            target: None,
            up: None,
            fov: None,
            near: None,
            far: None,
        }
    }

    pub fn viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = Some(viewport);
        self
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = Some(position);
        self
    }

    pub fn target(mut self, target: Vec3) -> Self {
        self.target = Some(target);
        self
    }

    pub fn up(mut self, up: Vec3) -> Self {
        self.up = Some(up);
        self
    }

    pub fn fov(mut self, fov: Deg<f32>) -> Self {
        self.fov = Some(fov);
        self
    }

    pub fn near(mut self, near: f32) -> Self {
        self.near = Some(near);
        self
    }

    pub fn far(mut self, far: f32) -> Self {
        self.far = Some(far);
        self
    }

    pub fn build(self) -> Result<Camera, crate::error::Error> {
        Ok(Camera::new_perspective(
            self.viewport
                .ok_or(crate::error::Error::FieldMissing("MissingViewport".into()))?,
            self.position
                .ok_or(crate::error::Error::FieldMissing("MissingPosition".into()))?,
            self.target
                .ok_or(crate::error::Error::FieldMissing("MissingTarget".into()))?,
            self.up
                .ok_or(crate::error::Error::FieldMissing("MissingUp".into()))?,
            self.fov
                .ok_or(crate::error::Error::FieldMissing("MissingFov".into()))?,
            self.near
                .ok_or(crate::error::Error::FieldMissing("MissingNear".into()))?,
            self.far
                .ok_or(crate::error::Error::FieldMissing("MissingFar".into()))?,
        ))
    }
}
