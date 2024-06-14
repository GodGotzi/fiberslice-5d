use glam::{vec3, Vec3};
use log::info;

use crate::{
    api::math::DirectMul,
    model::mesh::{Mesh, Vertices, WithOffset},
};

use super::{path::PathModul, DisplaySettings};

#[derive(Debug, Clone)]
pub struct ProfileCross {
    pub up: Vec3,
    pub down: Vec3,
    pub left: Vec3,
    pub right: Vec3,
}

impl ProfileCross {
    pub fn from_direction(
        direction: Vec3,
        (horizontal_radius, vertical_radius): (f32, f32),
    ) -> Self {
        let horizontal = if direction.z.abs() > 0.0 {
            direction.cross(Vec3::X)
        } else {
            direction.cross(Vec3::Z)
        };

        let vertical = direction.cross(horizontal);

        let normal_vertical = vertical.normalize();
        let normal_horizontal = horizontal.normalize();

        Self {
            up: vertical.normalize().direct_mul(&vec3(
                vertical_radius,
                vertical_radius,
                vertical_radius,
            )),
            down: vertical.normalize().direct_mul(&vec3(
                -vertical_radius,
                -vertical_radius,
                -vertical_radius,
            )),
            left: horizontal.normalize().direct_mul(&vec3(
                horizontal_radius,
                horizontal_radius,
                horizontal_radius,
            )),
            right: horizontal.normalize().direct_mul(&vec3(
                -horizontal_radius,
                -horizontal_radius,
                -horizontal_radius,
            )),
        }
    }
}

impl WithOffset for ProfileCross {
    fn with_offset(&self, offset: Vec3) -> Self {
        Self {
            up: self.up + offset,
            down: self.down + offset,
            left: self.left + offset,
            right: self.right + offset,
        }
    }
}

impl Mesh for ProfileCross {
    fn to_vertices(&self) -> Vertices {
        vec![
            self.up, self.right, self.down, self.up, self.down, self.left,
        ]
    }

    fn to_vertices_flipped(&self) -> Vertices {
        vec![
            self.up, self.down, self.right, self.up, self.left, self.down,
        ]
    }
}

pub struct Cuboid {
    profile_start: ProfileCross,
    profile_end: ProfileCross,
}

impl Cuboid {
    pub fn from_profiles(profile_start: ProfileCross, profile_end: ProfileCross) -> Self {
        Self {
            profile_start,
            profile_end,
        }
    }
}

impl Mesh for Cuboid {
    fn to_vertices(&self) -> Vertices {
        vec![
            // asdasd
            self.profile_start.up,
            self.profile_end.up,
            self.profile_end.right,
            self.profile_start.right,
            self.profile_start.up,
            self.profile_end.right,
            // asdasd
            self.profile_start.down,
            self.profile_end.right,
            self.profile_end.down,
            self.profile_start.down,
            self.profile_start.right,
            self.profile_end.right,
            // asdasd
            self.profile_start.down,
            self.profile_end.down,
            self.profile_end.left,
            self.profile_start.left,
            self.profile_start.down,
            self.profile_end.left,
            // asdasd
            self.profile_start.up,
            self.profile_end.left,
            self.profile_end.up,
            self.profile_start.up,
            self.profile_start.left,
            self.profile_end.left,
        ]
    }

    fn to_vertices_flipped(&self) -> Vertices {
        panic!("Not implemented")
    }
}

pub struct CuboidConnection {
    profile_start: ProfileCross,
    profile_end: ProfileCross,
}

impl CuboidConnection {
    pub fn from_profiles(profile_start: ProfileCross, profile_end: ProfileCross) -> Self {
        Self {
            profile_start,
            profile_end,
        }
    }
}

impl Mesh for CuboidConnection {
    fn to_vertices(&self) -> Vertices {
        vec![
            self.profile_start.up,
            self.profile_end.right,
            self.profile_start.right,
            // asdasd
            self.profile_start.down,
            self.profile_start.right,
            self.profile_end.right,
            // asdasd
            self.profile_start.down,
            self.profile_end.left,
            self.profile_start.left,
            // asdasd
            self.profile_start.up,
            self.profile_start.left,
            self.profile_end.left,
        ]
    }
}

impl PathModul {
    pub(super) fn to_vertices(&self, settings: &DisplaySettings) -> (Vertices, Vec<usize>) {
        let mut vertices = Vec::new();
        let mut offsets: Vec<usize> = Vec::new();

        let mut last_cross: Option<ProfileCross> = None;

        for (index, line) in self.lines.iter().enumerate() {
            let direction = line.direction();

            let profile = ProfileCross::from_direction(
                direction,
                (settings.horizontal / 2.0, settings.vertical / 2.0),
            );

            let profile_start = profile.with_offset(line.start);
            let profile_end = profile.with_offset(line.end);

            if index == self.lines.len() - 1 {
                vertices.extend_from_slice(&profile_end.to_vertices_flipped());
                offsets.push(vertices.len());
            }

            if line.print {
                if let Some(last) = last_cross.take() {
                    vertices.extend_from_slice(
                        &CuboidConnection::from_profiles(last, profile_start.clone()).to_vertices(),
                    );
                } else {
                    vertices.extend_from_slice(&profile_start.to_vertices());
                }

                vertices.extend_from_slice(
                    &Cuboid::from_profiles(profile_start, profile_end.clone()).to_vertices(),
                );
                last_cross = Some(profile_end);
            } else if let Some(last) = last_cross.take() {
                vertices.extend_from_slice(&last.to_vertices_flipped());

                offsets.push(vertices.len());
            }
        }

        (vertices, offsets)
    }
}
