use glam::{vec3, Vec3};
use log::info;

use crate::{
    api::{math::DirectMul, Reverse},
    model::{
        mesh::{Mesh, Vertices, WithOffset},
        shapes::Rect3d,
    },
};

use super::{path::PathModul, DisplaySettings};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrientation {
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

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
        let horizontal = if direction != Vec3::Z {
            direction.cross(Vec3::Z)
        } else {
            direction.cross(Vec3::X)
        };

        // turn horizontal vector to the right direction on direction vector
        let vertical = direction.cross(horizontal);

        info!("Direction: {:?}", direction);

        info!("Horizontal: {:?}", horizontal);
        info!("Vertical: {:?}", vertical);

        let normal_vertical = vertical.normalize();
        let normal_horizontal = horizontal.normalize();

        info!("Normal Horizontal: {:?}", normal_horizontal);
        info!("Normal Vertical: {:?}", normal_vertical);

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
        let mut vertices = Vec::new();

        vertices.extend_from_slice(&self.profile_start.to_vertices_flipped());
        vertices.extend_from_slice(&self.profile_end.to_vertices());

        vertices
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

    fn to_vertices_flipped(&self) -> Vertices {
        let mut vertices = Vec::new();

        vertices.extend_from_slice(&self.profile_start.to_vertices_flipped());
        vertices.extend_from_slice(&self.profile_end.to_vertices());

        vertices
    }
}

fn has_flipped_faces(direction: Vec3) -> bool {
    adjust_pane(direction.x, direction.y)
}

fn adjust_pane(x: f32, y: f32) -> bool {
    let alpha = (x / (y * y + x * x).sqrt()).asin().to_degrees();

    if (-45.0..=45.0).contains(&alpha) {
        y > 0.0
    } else {
        x < 0.0
    }
}

impl PathModul {
    pub(super) fn to_vertices(
        &self,
        settings: &DisplaySettings,
        layer: usize,
    ) -> (Vertices, Vec<usize>) {
        let mut vertices = Vec::new();
        let mut offsets: Vec<usize> = Vec::new();

        let mut last_cross: Option<ProfileCross> = None;

        for (index, line) in self.paths.iter().enumerate() {
            let direction = line.direction();

            let profile = ProfileCross::from_direction(
                direction,
                (settings.horizontal / 2.0, settings.vertical / 2.0),
            );

            let profile_start = profile.with_offset(line.start);
            let profile_end = profile.with_offset(line.end);

            if index == self.paths.len() - 1 {
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

pub(super) fn draw_path(
    vertices: &mut Vertices,
    path: (Vec3, Vec3),
    flip: bool,
    cross: &ProfileCross,
) {
    draw_rect_path(
        vertices,
        Rect3d {
            left_0: cross.up + path.0,
            left_1: cross.right + path.0,
            right_0: cross.up + path.1,
            right_1: cross.right + path.1,
        },
        flip,
        PathOrientation::SouthWest,
    );

    draw_rect_path(
        vertices,
        Rect3d {
            left_0: cross.down + path.0,
            left_1: cross.right + path.0,
            right_0: cross.down + path.1,
            right_1: cross.right + path.1,
        },
        flip,
        PathOrientation::NorthWest,
    );

    draw_rect_path(
        vertices,
        Rect3d {
            left_0: cross.down + path.0,
            left_1: cross.left + path.0,
            right_0: cross.down + path.1,
            right_1: cross.left + path.1,
        },
        flip,
        PathOrientation::NorthEast,
    );

    draw_rect_path(
        vertices,
        Rect3d {
            left_0: cross.up + path.0,
            left_1: cross.left + path.0,
            right_0: cross.up + path.1,
            right_1: cross.left + path.1,
        },
        flip,
        PathOrientation::SouthEast,
    );
}

pub(super) fn draw_cross_connection(
    vertices: &mut Vertices,
    center: &Vec3,
    start_cross: &ProfileCross,
    end_cross: &ProfileCross,
) {
    vertices.push(end_cross.up + *center);
    vertices.push(end_cross.right + *center);
    vertices.push(start_cross.right + *center);

    vertices.push(end_cross.up + *center);
    vertices.push(end_cross.left + *center);
    vertices.push(start_cross.left + *center);

    vertices.push(end_cross.down + *center);
    vertices.push(end_cross.right + *center);
    vertices.push(start_cross.right + *center);

    vertices.push(end_cross.down + *center);
    vertices.push(end_cross.left + *center);
    vertices.push(start_cross.left + *center);
}

pub(super) fn draw_rect_path(
    vertices: &mut Vertices,
    rect: Rect3d,
    face_flip: bool,
    orienation: PathOrientation,
) {
    let (mut triangle1, mut triangle2) = match orienation {
        PathOrientation::SouthEast => {
            let triangle1 = (rect.right_0, rect.left_1, rect.left_0);
            let triangle2 = (rect.right_0, rect.right_1, rect.left_1);

            (triangle1, triangle2)
        }
        PathOrientation::SouthWest => {
            let triangle1 = (rect.left_0, rect.left_1, rect.right_1);
            let triangle2 = (rect.right_1, rect.right_0, rect.left_0);

            (triangle1, triangle2)
        }
        PathOrientation::NorthEast => {
            let triangle1 = (rect.left_0, rect.left_1, rect.right_0);
            let triangle2 = (rect.left_1, rect.right_1, rect.right_0);

            (triangle1, triangle2)
        }
        PathOrientation::NorthWest => {
            let triangle1 = (rect.left_0, rect.right_0, rect.right_1);
            let triangle2 = (rect.right_1, rect.left_1, rect.left_0);

            (triangle1, triangle2)
        }
    };

    if face_flip {
        triangle1.reverse();
        triangle2.reverse();
    }

    vertices.push(triangle1.0);
    vertices.push(triangle1.1);
    vertices.push(triangle1.2);

    vertices.push(triangle2.0);
    vertices.push(triangle2.1);
    vertices.push(triangle2.2);
}

pub(super) fn draw_rect(
    vertices: &mut Vertices,
    point_left_0: Vec3,
    point_left_1: Vec3,
    point_right_0: Vec3,
    point_right_1: Vec3,
) {
    vertices.push(point_left_0);
    vertices.push(point_left_1);
    vertices.push(point_right_0);

    vertices.push(point_left_1);
    vertices.push(point_right_1);
    vertices.push(point_right_0);
}

pub(super) fn draw_rect_with_cross(vertices: &mut Vertices, center: &Vec3, cross: &ProfileCross) {
    draw_rect(
        vertices,
        cross.up + *center,
        cross.right + *center,
        cross.down + *center,
        cross.left + *center,
    );
}
