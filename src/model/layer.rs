use std::cell::{Cell, RefCell};

use bevy::{math::vec3, prelude::Vec3};

use super::gcode::state::State;

pub struct PartCoordinator<'a> {
    mesh: RefCell<&'a mut LayerMesh<'a>>,
    offset_start: Cell<usize>,
    offset_end: Cell<usize>,
    offset_part_start: Cell<usize>,
    offset_part_end: Cell<usize>,
}

enum PathOrientation {
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

pub fn push_position(mesh: &mut MeshPart, position: [f32; 3]) {
    mesh.positions.push(position);
}

pub fn push_color(mesh: &mut MeshPart, color: [f32; 4]) {
    mesh.colors.push(color);
}

pub fn push_normal(mesh: &mut MeshPart, normal: [f32; 3]) {
    mesh.normals.push(normal);
}

pub struct WVec3(Vec3);

impl From<WVec3> for [f32; 3] {
    fn from(value: WVec3) -> Self {
        [value.0.x, value.0.z, value.0.y]
    }
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
        push_position(&mut self.mesh.borrow_mut().mesh, WVec3(triangle.0).into());
        push_position(&mut self.mesh.borrow_mut().mesh, WVec3(triangle.1).into());
        push_position(&mut self.mesh.borrow_mut().mesh, WVec3(triangle.2).into());

        push_color(&mut self.mesh.borrow_mut().mesh, *color);
        push_color(&mut self.mesh.borrow_mut().mesh, *color);
        push_color(&mut self.mesh.borrow_mut().mesh, *color);

        let normal = (triangle.1 - triangle.0)
            .cross(triangle.2 - triangle.0)
            .normalize();

        push_normal(&mut self.mesh.borrow_mut().mesh, WVec3(normal).into());
        push_normal(&mut self.mesh.borrow_mut().mesh, WVec3(normal).into());
        push_normal(&mut self.mesh.borrow_mut().mesh, WVec3(normal).into());

        self.offset_end.replace(self.offset_end.get() + 3);
        self.offset_part_end.replace(self.offset_part_end.get() + 3);
    }

    pub fn next_part_meshref(&self, state: State, line_range: (usize, usize)) -> Result<(), ()> {
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

            Ok(())
        }
    }

    pub fn finish(&self) -> Result<(), ()> {
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

            self.mesh.borrow_mut().child_models.last_mut().unwrap().main = Some(meshref);

            Ok(())
        }
    }
}

pub fn draw_path(
    path: (Vec3, Vec3),
    color: &[f32; 4],
    coordinator: &PartCoordinator,
    cross: &Cross,
) {
    draw_rect_path(
        cross.up + path.0,
        cross.right + path.0,
        cross.up + path.1,
        cross.right + path.1,
        color,
        PathOrientation::SouthWest,
        coordinator,
    );

    draw_rect_path(
        cross.down + path.0,
        cross.right + path.0,
        cross.down + path.1,
        cross.right + path.1,
        color,
        PathOrientation::NorthWest,
        coordinator,
    );

    draw_rect_path(
        cross.down + path.0,
        cross.left + path.0,
        cross.down + path.1,
        cross.left + path.1,
        color,
        PathOrientation::NorthEast,
        coordinator,
    );

    draw_rect_path(
        cross.up + path.0,
        cross.left + path.0,
        cross.up + path.1,
        cross.left + path.1,
        color,
        PathOrientation::SouthEast,
        coordinator,
    );
}

pub fn draw_cross_connection(
    center: &Vec3,
    start_cross: &Cross,
    end_cross: &Cross,
    color: &[f32; 4],
    coordinator: &PartCoordinator,
) {
    coordinator.add_triangle(
        (
            end_cross.up + *center,
            end_cross.right + *center,
            start_cross.right + *center,
        ),
        color,
    );

    coordinator.add_triangle(
        (
            end_cross.up + *center,
            end_cross.left + *center,
            start_cross.left + *center,
        ),
        color,
    );

    coordinator.add_triangle(
        (
            end_cross.down + *center,
            end_cross.right + *center,
            start_cross.right + *center,
        ),
        color,
    );

    coordinator.add_triangle(
        (
            end_cross.down + *center,
            end_cross.left + *center,
            start_cross.left + *center,
        ),
        color,
    );
}

fn draw_rect_path(
    point_left_0: Vec3,
    point_left_1: Vec3,
    point_right_0: Vec3,
    point_right_1: Vec3,
    color: &[f32; 4],
    orienation: PathOrientation,
    coordinator: &PartCoordinator,
) {
    match orienation {
        PathOrientation::SouthEast => {
            coordinator.add_triangle((point_right_0, point_left_1, point_left_0), color);

            coordinator.add_triangle((point_right_0, point_right_1, point_left_1), color);
        }
        PathOrientation::SouthWest => {
            coordinator.add_triangle((point_left_0, point_left_1, point_right_1), color);

            coordinator.add_triangle((point_right_1, point_right_0, point_left_0), color);
        }
        PathOrientation::NorthEast => {
            coordinator.add_triangle((point_left_0, point_left_1, point_right_0), color);

            coordinator.add_triangle((point_left_1, point_right_1, point_right_0), color);
        }
        PathOrientation::NorthWest => {
            coordinator.add_triangle((point_left_0, point_right_0, point_right_1), color);

            coordinator.add_triangle((point_right_1, point_left_1, point_left_0), color);
        }
    }
}

pub fn draw_rect(
    point_left_0: Vec3,
    point_left_1: Vec3,
    point_right_0: Vec3,
    point_right_1: Vec3,
    color: &[f32; 4],
    coordinator: &PartCoordinator,
) {
    coordinator.add_triangle((point_left_0, point_left_1, point_right_0), color);

    coordinator.add_triangle((point_left_1, point_right_1, point_right_0), color);
}

pub fn draw_rect_with_cross(
    center: &Vec3,
    cross: &Cross,
    color: &[f32; 4],
    coordinator: &PartCoordinator,
) {
    draw_rect(
        cross.up + *center,
        cross.right + *center,
        cross.down + *center,
        cross.left + *center,
        color,
        coordinator,
    );
}

#[derive(Debug)]
pub struct Cross {
    up: Vec3,
    down: Vec3,
    left: Vec3,
    right: Vec3,
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
