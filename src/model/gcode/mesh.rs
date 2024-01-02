use three_d::{vec3, InnerSpace, Vector3};

use crate::{
    api::{math::DirectMul, Reverse},
    model::{mesh::Vertices, shapes::Rect3d},
};

use super::{path::PathModul, DisplaySettings};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrientation {
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

#[derive(Debug)]
pub struct ProfileCross {
    pub up: Vector3<f32>,
    pub down: Vector3<f32>,
    pub left: Vector3<f32>,
    pub right: Vector3<f32>,
}

impl ProfileCross {
    pub fn from_direction(direction: Vector3<f32>, radius: f32) -> Self {
        let horizontal = direction.cross(vec3(0.0, 0.0, direction.z + 1.0));
        let vertical = direction.cross(vec3(direction.x + 1.0, direction.y + 1.0, 0.0));

        Self {
            up: vertical
                .normalize()
                .direct_mul(&vec3(radius, radius, radius)),
            down: vertical
                .normalize()
                .direct_mul(&vec3(-radius, -radius, -radius)),
            left: horizontal
                .normalize()
                .direct_mul(&vec3(radius, radius, radius)),
            right: horizontal
                .normalize()
                .direct_mul(&vec3(-radius, -radius, -radius)),
        }
    }
}

fn has_flipped_faces(direction: Vector3<f32>) -> bool {
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
    pub(super) fn to_vertices(self, settings: &DisplaySettings) -> (Vertices, Vec<usize>) {
        let mut vertices = Vec::new();
        let mut offsets: Vec<usize> = Vec::new();

        let mut last_cross: Option<ProfileCross> = None;

        for element in self.paths.iter().enumerate() {
            let path = element.1;

            if element.0 == self.paths.len() - 1 {
                if let Some(last) = last_cross.take() {
                    draw_rect_with_cross(&mut vertices, &path.end, &last);
                    offsets.push(vertices.len());
                }
            }

            if path.print {
                let direction = path.direction();

                let cross = ProfileCross::from_direction(direction, settings.diameter / 2.0);

                if let Some(last) = last_cross.take() {
                    draw_cross_connection(&mut vertices, &path.start, &cross, &last);
                } else {
                    draw_rect_with_cross(&mut vertices, &path.start, &cross);
                }

                let flip_faces = !has_flipped_faces(direction);

                draw_path(&mut vertices, (path.start, path.end), !flip_faces, &cross);
                last_cross = Some(cross);
            } else if let Some(last) = last_cross.take() {
                draw_rect_with_cross(&mut vertices, &path.end, &last);
                offsets.push(vertices.len());
            }
        }

        (vertices, offsets)
    }
}

pub(super) fn draw_path(
    vertices: &mut Vertices,
    path: (Vector3<f32>, Vector3<f32>),
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
    center: &Vector3<f32>,
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
    point_left_0: Vector3<f32>,
    point_left_1: Vector3<f32>,
    point_right_0: Vector3<f32>,
    point_right_1: Vector3<f32>,
) {
    vertices.push(point_left_0);
    vertices.push(point_left_1);
    vertices.push(point_right_0);

    vertices.push(point_left_1);
    vertices.push(point_right_1);
    vertices.push(point_right_0);
}

pub(super) fn draw_rect_with_cross(
    vertices: &mut Vertices,
    center: &Vector3<f32>,
    cross: &ProfileCross,
) {
    draw_rect(
        vertices,
        cross.up + *center,
        cross.right + *center,
        cross.down + *center,
        cross.left + *center,
    );
}
