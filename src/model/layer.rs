use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use bevy::prelude::Mesh;
use three_d_asset::{vec3, InnerSpace, Srgba, Vector3};

use super::gcode::state::State;

pub struct PartCoordinator<'a> {
    mesh: RefCell<&'a mut LayerMesh<'a>>,
    offset_start: Cell<usize>,
    offset_end: Cell<usize>,
    offset_part_start: Cell<usize>,
    offset_part_end: Cell<usize>,
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

pub struct WSrgba(pub Srgba);

impl From<WSrgba> for [f32; 4] {
    fn from(value: WSrgba) -> Self {
        [
            value.0.r as f32 / 255.0,
            value.0.g as f32 / 255.0,
            value.0.b as f32 / 255.0,
            value.0.a as f32 / 255.0,
        ]
    }
}

pub struct WVector3(Vector3<f64>);

impl From<WVector3> for [f32; 3] {
    fn from(value: WVector3) -> Self {
        [value.0.x as f32, value.0.z as f32, value.0.y as f32]
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

    pub fn add_triangle(&self, triangle: (Vector3<f64>, Vector3<f64>, Vector3<f64>), color: Srgba) {
        push_position(
            &mut self.mesh.borrow_mut().mesh,
            WVector3(triangle.0).into(),
        );
        push_position(
            &mut self.mesh.borrow_mut().mesh,
            WVector3(triangle.1).into(),
        );
        push_position(
            &mut self.mesh.borrow_mut().mesh,
            WVector3(triangle.2).into(),
        );

        push_color(&mut self.mesh.borrow_mut().mesh, WSrgba(color).into());
        push_color(&mut self.mesh.borrow_mut().mesh, WSrgba(color).into());
        push_color(&mut self.mesh.borrow_mut().mesh, WSrgba(color).into());

        let normal = (triangle.1 - triangle.0)
            .cross(triangle.2 - triangle.0)
            .normalize();

        push_normal(&mut self.mesh.borrow_mut().mesh, WVector3(normal).into());
        push_normal(&mut self.mesh.borrow_mut().mesh, WVector3(normal).into());
        push_normal(&mut self.mesh.borrow_mut().mesh, WVector3(normal).into());

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
    path: (Vector3<f64>, Vector3<f64>),
    color: &Srgba,
    coordinator: &PartCoordinator,
    cross: &Cross,
) {
    draw_rect(
        cross.up + path.0,
        cross.right + path.0,
        cross.up + path.1,
        cross.right + path.1,
        color,
        coordinator,
    );

    draw_rect(
        cross.down + path.0,
        cross.right + path.0,
        cross.down + path.1,
        cross.right + path.1,
        color,
        coordinator,
    );

    draw_rect(
        cross.down + path.0,
        cross.left + path.0,
        cross.down + path.1,
        cross.left + path.1,
        color,
        coordinator,
    );

    draw_rect(
        cross.up + path.0,
        cross.left + path.0,
        cross.up + path.1,
        cross.left + path.1,
        color,
        coordinator,
    );
}

pub fn draw_cross_connection(
    center: &Vector3<f64>,
    start_cross: &Cross,
    end_cross: &Cross,
    color: &Srgba,
    coordinator: &PartCoordinator,
) {
    coordinator.add_triangle(
        (
            end_cross.up + center,
            end_cross.right + center,
            start_cross.right + center,
        ),
        *color,
    );

    coordinator.add_triangle(
        (
            end_cross.up + center,
            end_cross.left + center,
            start_cross.left + center,
        ),
        *color,
    );

    coordinator.add_triangle(
        (
            end_cross.down + center,
            end_cross.right + center,
            start_cross.right + center,
        ),
        *color,
    );

    coordinator.add_triangle(
        (
            end_cross.down + center,
            end_cross.left + center,
            start_cross.left + center,
        ),
        *color,
    );
}

pub fn draw_rect(
    point_left_0: Vector3<f64>,
    point_left_1: Vector3<f64>,
    point_right_0: Vector3<f64>,
    point_right_1: Vector3<f64>,
    color: &Srgba,
    coordinator: &PartCoordinator,
) {
    coordinator.add_triangle((point_left_0, point_left_1, point_right_0), *color);

    coordinator.add_triangle((point_right_0, point_right_1, point_left_1), *color);
}

pub fn draw_rect_with_cross(
    center: &Vector3<f64>,
    cross: &Cross,
    color: &Srgba,
    coordinator: &PartCoordinator,
) {
    draw_rect(
        cross.up + center,
        cross.right + center,
        cross.down + center,
        cross.left + center,
        color,
        coordinator,
    );
}

#[derive(Debug)]
pub struct Cross {
    up: Vector3<f64>,
    down: Vector3<f64>,
    left: Vector3<f64>,
    right: Vector3<f64>,
}

pub fn get_cross(direction: Vector3<f64>, radius: f64) -> Cross {
    let horizontal = direction.cross(vec3(0.0, 0.0, direction.z + 1.0));
    let vertical = direction.cross(vec3(direction.x + 1.0, direction.y + 1.0, 0.0));

    Cross {
        up: vertical.normalize() * radius,
        down: vertical.normalize() * (-radius),
        left: horizontal.normalize() * radius,
        right: horizontal.normalize() * (-radius),
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

pub struct ToolPathModel<'a> {
    pub layers: HashMap<usize, RefCell<LayerMesh<'a>>>,
    pub mesh: Mesh,
}
