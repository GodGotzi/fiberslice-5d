use std::cell::{Cell, RefCell};

use bevy::{
    math::vec3,
    prelude::{Color, Vec3},
};

use crate::{math::FSVec3, utils::Flip};

use super::{
    gcode::{state::State, toolpath::PathModul},
    shapes::Rect3d,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrientation {
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

#[derive(Debug)]
pub struct Cross {
    pub up: Vec3,
    pub down: Vec3,
    pub left: Vec3,
    pub right: Vec3,
}

pub fn get_cross(direction: Vec3, radius: f32) -> Cross {
    let horizontal = direction.cross(vec3(0.0, 0.0, direction.z + 1.0));
    let vertical = direction.cross(vec3(direction.x + 1.0, direction.y + 1.0, 0.0));

    Cross {
        up: vertical.normalize() * vec3(radius, radius, radius),
        down: vertical.normalize() * vec3(-radius, -radius, -radius),
        left: horizontal.normalize() * vec3(radius, radius, radius),
        right: horizontal.normalize() * vec3(-radius, -radius, -radius),
    }
}

pub struct LayerMesh<'a> {
    pub mesh: MeshPart,
    pub line_range: Option<(usize, usize)>,
    pub child_models: Vec<LayerPart<'a>>,
}

impl<'a> LayerMesh<'a> {
    pub fn empty() -> Self {
        Self {
            mesh: MeshPart {
                positions: Vec::new(),
                normals: Vec::new(),
                colors: Vec::new(),
            },
            line_range: None,
            child_models: Vec::new(),
        }
    }
}

pub struct MeshPart {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
}

impl MeshPart {
    pub fn push_position(&mut self, position: [f32; 3]) {
        self.positions.push(position);
    }

    pub fn push_color(&mut self, color: [f32; 4]) {
        self.colors.push(color);
    }

    pub fn push_normal(&mut self, normal: [f32; 3]) {
        self.normals.push(normal);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LayerPart<'a> {
    pub main: Option<MeshRef<'a>>,
    state: State,
    line_range: (usize, usize),
    child_meshes: Vec<MeshRef<'a>>,
}

impl<'a> LayerPart<'a> {
    pub fn new(state: State, line_range: (usize, usize)) -> Self {
        Self {
            main: None,
            state,
            line_range,
            child_meshes: Vec::new(),
        }
    }
}

impl<'a> LayerPart<'a> {
    pub fn push_child(&mut self, child: MeshRef<'a>) {
        self.child_meshes.push(child);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MeshRef<'a> {
    pub positions: &'a [[f32; 3]],
    colors: &'a [[f32; 4]],
    normals: &'a [[f32; 3]],
    start: usize,
    end: usize,
}

fn adjust_faces(direction: Vec3) -> bool {
    !adjust_pane(direction.x, direction.y)
}

fn adjust_pane(x: f32, y: f32) -> bool {
    let alpha = (x / (y * y + x * x).sqrt()).asin().to_degrees();

    if (-45.0..=45.0).contains(&alpha) {
        y > 0.0
    } else {
        x < 0.0
    }
}

pub struct PartCoordinator<'a> {
    mesh: RefCell<&'a mut LayerMesh<'a>>,
    offset_start: Cell<usize>,
    offset_end: Cell<usize>,
    offset_part_start: Cell<usize>,
    offset_part_end: Cell<usize>,
}

impl<'a> PartCoordinator<'a> {
    pub fn new(mesh: &'a mut LayerMesh<'a>) -> Self {
        Self {
            mesh: RefCell::new(mesh),
            offset_start: Cell::new(0),
            offset_end: Cell::new(0),
            offset_part_start: Cell::new(0),
            offset_part_end: Cell::new(0),
        }
    }

    pub fn add_triangle(&self, triangle: (Vec3, Vec3, Vec3), color: &[f32; 4]) {
        let mesh = &mut self.mesh.borrow_mut().mesh;
        mesh.push_position(FSVec3(triangle.0).into());
        mesh.push_position(FSVec3(triangle.1).into());
        mesh.push_position(FSVec3(triangle.2).into());

        mesh.push_color(*color);
        mesh.push_color(*color);
        mesh.push_color(*color);

        let normal = (triangle.1 - triangle.0)
            .cross(triangle.2 - triangle.0)
            .normalize();

        mesh.push_normal(FSVec3(normal).into());
        mesh.push_normal(FSVec3(normal).into());
        mesh.push_normal(FSVec3(normal).into());

        self.offset_end.replace(self.offset_end.get() + 3);
        self.offset_part_end.replace(self.offset_part_end.get() + 3);
    }

    pub fn finished_child(&self, state: State, line_range: (usize, usize)) {
        let start = self.offset_part_start.get();
        let end = self.offset_part_end.get();

        self.offset_part_start.replace(end);

        unsafe {
            let meshref = MeshRef {
                positions: &self.mesh.as_ptr().as_ref().unwrap().mesh.positions[start..end],
                colors: &self.mesh.as_ptr().as_ref().unwrap().mesh.colors[start..end],
                normals: &self.mesh.as_ptr().as_ref().unwrap().mesh.normals[start..end],
                start,
                end,
            };

            if self.mesh.borrow().child_models.last().is_none() {
                self.mesh
                    .borrow_mut()
                    .child_models
                    .push(LayerPart::new(state, line_range));
            }

            self.mesh
                .borrow_mut()
                .child_models
                .last_mut()
                .unwrap()
                .push_child(meshref);
        }
    }

    pub fn finish(&self) {
        let start = self.offset_start.get();
        let end = self.offset_end.get();

        self.offset_start.replace(end);

        unsafe {
            let meshref = MeshRef {
                positions: &self.mesh.as_ptr().as_ref().unwrap().mesh.positions[start..end],
                colors: &self.mesh.as_ptr().as_ref().unwrap().mesh.colors[start..end],
                normals: &self.mesh.as_ptr().as_ref().unwrap().mesh.normals[start..end],
                start,
                end,
            };

            if let Some(last) = self.mesh.borrow_mut().child_models.last_mut() {
                last.main = Some(meshref);
            }
        }
    }

    pub fn compute_model(&self, path_modul: &PathModul) {
        let diameter = 0.45;
        let mut last_cross: Option<Cross> = None;

        let color = path_modul
            .state
            .print_type
            .as_ref()
            .unwrap_or(&crate::slicer::print_type::PrintType::Unknown)
            .get_color()
            .as_rgba_f32();

        for element in path_modul.paths.iter().enumerate() {
            let path = element.1;

            if element.0 == path_modul.paths.len() - 1 {
                if let Some(last) = last_cross.take() {
                    self.draw_rect_with_cross(&path.end, &last, &color);

                    self.finished_child(path_modul.state.clone(), path_modul.line_range);
                }
            }

            if path.print {
                let direction = path.direction();

                let cross = get_cross(direction, diameter / 2.0);

                if let Some(last) = last_cross.take() {
                    self.draw_cross_connection(&path.start, &cross, &last, &color);
                } else {
                    self.draw_rect_with_cross(&path.start, &cross, &color);
                }

                let flip = !adjust_faces(direction);

                self.draw_path((path.start, path.end), &color, !flip, &cross);
                last_cross = Some(cross);
            } else if let Some(last) = last_cross.take() {
                self.draw_rect_with_cross(&path.end, &last, &color);

                self.finished_child(path_modul.state.clone(), path_modul.line_range);
            }
        }
    }

    pub fn draw_path(&self, path: (Vec3, Vec3), color: &[f32; 4], flip: bool, cross: &Cross) {
        self.draw_rect_path(
            Rect3d {
                left_0: cross.up + path.0,
                left_1: cross.right + path.0,
                right_0: cross.up + path.1,
                right_1: cross.right + path.1,
            },
            color,
            flip,
            PathOrientation::SouthWest,
        );

        self.draw_rect_path(
            Rect3d {
                left_0: cross.down + path.0,
                left_1: cross.right + path.0,
                right_0: cross.down + path.1,
                right_1: cross.right + path.1,
            },
            color,
            flip,
            PathOrientation::NorthWest,
        );

        self.draw_rect_path(
            Rect3d {
                left_0: cross.down + path.0,
                left_1: cross.left + path.0,
                right_0: cross.down + path.1,
                right_1: cross.left + path.1,
            },
            color,
            flip,
            PathOrientation::NorthEast,
        );

        self.draw_rect_path(
            Rect3d {
                left_0: cross.up + path.0,
                left_1: cross.left + path.0,
                right_0: cross.up + path.1,
                right_1: cross.left + path.1,
            },
            color,
            flip,
            PathOrientation::SouthEast,
        );
    }

    pub fn draw_cross_connection(
        &self,
        center: &Vec3,
        start_cross: &Cross,
        end_cross: &Cross,
        color: &[f32; 4],
    ) {
        self.add_triangle(
            (
                end_cross.up + *center,
                end_cross.right + *center,
                start_cross.right + *center,
            ),
            color,
        );

        self.add_triangle(
            (
                end_cross.up + *center,
                end_cross.left + *center,
                start_cross.left + *center,
            ),
            color,
        );

        self.add_triangle(
            (
                end_cross.down + *center,
                end_cross.right + *center,
                start_cross.right + *center,
            ),
            color,
        );

        self.add_triangle(
            (
                end_cross.down + *center,
                end_cross.left + *center,
                start_cross.left + *center,
            ),
            color,
        );
    }

    fn draw_rect_path(
        &self,
        rect: Rect3d,
        color: &[f32; 4],
        flip: bool,
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

        if flip {
            triangle1.flip();
            triangle2.flip();
        }

        self.add_triangle(triangle1, color);
        self.add_triangle(triangle2, color);
    }

    pub fn draw_rect(
        &self,
        point_left_0: Vec3,
        point_left_1: Vec3,
        point_right_0: Vec3,
        point_right_1: Vec3,
        color: &[f32; 4],
    ) {
        self.add_triangle((point_left_0, point_left_1, point_right_0), color);

        self.add_triangle((point_left_1, point_right_1, point_right_0), color);
    }

    pub fn draw_rect_with_cross(&self, center: &Vec3, cross: &Cross, color: &[f32; 4]) {
        self.draw_rect(
            cross.up + *center,
            cross.right + *center,
            cross.down + *center,
            cross.left + *center,
            color,
        );
    }
}
