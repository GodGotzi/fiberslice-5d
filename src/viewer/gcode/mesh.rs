use glam::Vec3;

use crate::{
    geometry::{mesh::Mesh, QuadFace},
    picking::hitbox::Hitbox,
    prelude::SharedMut,
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

        Self {
            up: vertical.normalize() * vertical_radius,
            down: vertical.normalize() * -vertical_radius,
            left: horizontal.normalize() * horizontal_radius,
            right: horizontal.normalize() * -horizontal_radius,
        }
    }
}

impl ProfileCross {
    fn with_offset(&self, offset: Vec3) -> Self {
        Self {
            up: self.up + offset,
            down: self.down + offset,
            left: self.left + offset,
            right: self.right + offset,
        }
    }
}

/*
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
*/

impl Mesh<6> for ProfileCross {
    fn to_triangle_vertices(&self) -> [glam::Vec3; 6] {
        [
            self.up, self.right, self.down, self.up, self.down, self.left,
        ]
    }

    fn to_triangle_vertices_flipped(&self) -> [glam::Vec3; 6] {
        [
            self.up, self.down, self.right, self.up, self.left, self.down,
        ]
    }
}

pub struct PathMesh {
    profile_start: ProfileCross,
    profile_end: ProfileCross,
}

impl PathMesh {
    pub fn from_profiles(profile_start: ProfileCross, profile_end: ProfileCross) -> Self {
        Self {
            profile_start,
            profile_end,
        }
    }
}

impl Mesh<24> for PathMesh {
    fn to_triangle_vertices(&self) -> [glam::Vec3; 24] {
        [
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
}

impl From<PathMesh> for PathHitbox {
    fn from(val: PathMesh) -> Self {
        PathHitbox {
            north_west: QuadFace {
                normal: (val.profile_end.up - val.profile_start.up)
                    .cross(val.profile_start.right - val.profile_start.up),
                max: val
                    .profile_end
                    .up
                    .max(val.profile_start.up)
                    .max(val.profile_start.right)
                    .max(val.profile_end.right),
                min: val
                    .profile_end
                    .up
                    .min(val.profile_start.up)
                    .min(val.profile_start.right)
                    .min(val.profile_end.right),
            },
            north_east: QuadFace {
                normal: (val.profile_end.right - val.profile_start.right)
                    .cross(val.profile_start.down - val.profile_start.right),
                max: val
                    .profile_end
                    .right
                    .max(val.profile_start.right)
                    .max(val.profile_start.down)
                    .max(val.profile_end.down),
                min: val
                    .profile_end
                    .right
                    .min(val.profile_start.right)
                    .min(val.profile_start.down)
                    .min(val.profile_end.down),
            },
            south_west: QuadFace {
                normal: (val.profile_end.down - val.profile_start.down)
                    .cross(val.profile_start.left - val.profile_start.down),
                max: val
                    .profile_end
                    .down
                    .max(val.profile_start.down)
                    .max(val.profile_start.left)
                    .max(val.profile_end.left),
                min: val
                    .profile_end
                    .down
                    .min(val.profile_start.down)
                    .min(val.profile_start.left)
                    .min(val.profile_end.left),
            },
            south_east: QuadFace {
                normal: (val.profile_end.left - val.profile_start.left)
                    .cross(val.profile_start.up - val.profile_start.left),
                max: val
                    .profile_end
                    .left
                    .max(val.profile_start.left)
                    .max(val.profile_start.up)
                    .max(val.profile_end.up),
                min: val
                    .profile_end
                    .left
                    .min(val.profile_start.left)
                    .min(val.profile_start.up)
                    .min(val.profile_end.up),
            },
            enabled: true,
        }
    }
}

pub struct PathConnectionMesh {
    profile_start: ProfileCross,
    profile_end: ProfileCross,
}

impl PathConnectionMesh {
    pub fn from_profiles(profile_start: ProfileCross, profile_end: ProfileCross) -> Self {
        Self {
            profile_start,
            profile_end,
        }
    }
}

impl Mesh<12> for PathConnectionMesh {
    fn to_triangle_vertices(&self) -> [glam::Vec3; 12] {
        [
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
    pub(super) fn to_vertices(&self, settings: &DisplaySettings) -> (Vec<Vec3>, Vec<usize>) {
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
                vertices.extend_from_slice(&profile_end.to_triangle_vertices_flipped());
                offsets.push(vertices.len());
            }

            if line.print {
                if let Some(last) = last_cross.take() {
                    vertices.extend_from_slice(
                        &PathConnectionMesh::from_profiles(last, profile_start.clone())
                            .to_triangle_vertices(),
                    );
                } else {
                    vertices.extend_from_slice(&profile_start.to_triangle_vertices());
                }

                vertices.extend_from_slice(
                    &PathMesh::from_profiles(profile_start, profile_end.clone())
                        .to_triangle_vertices(),
                );
                last_cross = Some(profile_end);
            } else if let Some(last) = last_cross.take() {
                vertices.extend_from_slice(&last.to_triangle_vertices_flipped());

                offsets.push(vertices.len());
            }
        }

        (vertices, offsets)
    }
}

#[derive(Debug, Clone)]
pub struct PathHitbox {
    north_west: QuadFace,
    north_east: QuadFace,
    south_west: QuadFace,
    south_east: QuadFace,
    enabled: bool,
}

impl Hitbox for PathHitbox {
    fn check_hit(&self, ray: &crate::picking::ray::Ray) -> Option<f32> {
        let faces = [
            self.north_west,
            self.north_east,
            self.south_west,
            self.south_east,
        ];

        let mut min = None;

        for quad_face in faces {
            let distance = quad_face.check_hit(ray);

            if let Some(distance) = distance {
                if min.unwrap_or(f32::MAX) > distance || min.is_none() {
                    min = Some(distance);
                }
            }
        }

        min
    }

    fn expand(&mut self, _box: &SharedMut<Box<dyn Hitbox>>) {
        // Not expandable
        // TODO either figure out how to expand this or remove this method for this type or make it clear that this is not expandable
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn min(&self) -> Vec3 {
        self.north_west
            .min
            .min(self.north_east.min)
            .min(self.south_west.min)
            .min(self.south_east.min)
    }

    fn max(&self) -> Vec3 {
        self.north_west
            .max
            .max(self.north_east.max)
            .max(self.south_west.max)
            .max(self.south_east.max)
    }
}
