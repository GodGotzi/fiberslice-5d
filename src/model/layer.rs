use std::cell::{Cell, RefCell};

use three_d::CpuMesh;

use three_d::{Context, Gm, Mesh, PhysicalMaterial, RenderStates};
use three_d_asset::{vec3, InnerSpace, LightingModel, Positions, Srgba, TriMesh, Vector3};

use super::gcode::state::State;

pub struct PartCoordinator<'a> {
    mesh: RefCell<&'a mut LayerModel<'a>>,
    offset_start: Cell<usize>,
    offset_end: Cell<usize>,
    offset_part_start: Cell<usize>,
    offset_part_end: Cell<usize>,
}

pub fn push_position<'a>(mut mesh: &'a mut TriMesh, position: Vector3<f64>) -> Result<(), ()> {
    match &mut mesh.positions {
        Positions::F64(positions) => {
            positions.push(position);
            Ok(())
        }
        _ => Err(()),
    }
}

pub fn push_color<'a>(mut mesh: &'a mut TriMesh, color: Srgba) {
    let colors = mesh.colors.as_mut().unwrap();
    colors.push(color);
}

pub fn push_normal<'a>(mut mesh: &'a mut TriMesh, normal: Vector3<f32>) {
    let normals = mesh.normals.as_mut().unwrap();
    normals.push(normal);
}

impl<'a> PartCoordinator<'a> {
    pub fn new(mesh: &'a mut LayerModel<'a>) -> Self {
        Self {
            mesh: RefCell::new(mesh),
            offset_start: Cell::new(0),
            offset_end: Cell::new(0),
            offset_part_start: Cell::new(0),
            offset_part_end: Cell::new(0),
        }
    }

    pub fn add_triangle(&self, triangle: (Vector3<f64>, Vector3<f64>, Vector3<f64>), color: Srgba) {
        push_position(&mut self.mesh.borrow_mut().trimesh, triangle.0);
        push_position(&mut self.mesh.borrow_mut().trimesh, triangle.1);
        push_position(&mut self.mesh.borrow_mut().trimesh, triangle.2);

        push_color(&mut self.mesh.borrow_mut().trimesh, color);
        push_color(&mut self.mesh.borrow_mut().trimesh, color);
        push_color(&mut self.mesh.borrow_mut().trimesh, color);

        let normal_f64 = (triangle.1 - triangle.0)
            .cross(triangle.2 - triangle.0)
            .normalize();

        let normal = Vector3::new(
            normal_f64.x as f32,
            normal_f64.y as f32,
            normal_f64.z as f32,
        );

        push_normal(&mut self.mesh.borrow_mut().trimesh, normal);
        push_normal(&mut self.mesh.borrow_mut().trimesh, normal);
        push_normal(&mut self.mesh.borrow_mut().trimesh, normal);

        self.offset_end.replace(self.offset_end.get() + 3);
        self.offset_part_end.replace(self.offset_part_end.get() + 3);
    }

    pub fn next_part_meshref(&self, state: State, line_range: (usize, usize)) -> Result<(), ()> {
        let start = self.offset_part_start.get();
        let end = self.offset_part_end.get();

        self.offset_part_start.replace(end);

        unsafe {
            match &self.mesh.as_ptr().as_ref().unwrap().trimesh.positions {
                Positions::F64(positions) => {
                    let colors = self
                        .mesh
                        .as_ptr()
                        .as_ref()
                        .unwrap()
                        .trimesh
                        .colors
                        .as_ref()
                        .unwrap();

                    let normals = self
                        .mesh
                        .as_ptr()
                        .as_ref()
                        .unwrap()
                        .trimesh
                        .normals
                        .as_ref()
                        .unwrap();

                    let meshref = MeshRef {
                        positions: &positions[start..end],
                        colors: &colors[start..end],
                        normals: &normals[start..end],
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
                _ => Err(()),
            }
        }
    }

    pub fn finish(&self) -> Result<(), ()> {
        let start = self.offset_start.get();
        let end = self.offset_end.get();

        self.offset_start.replace(end);

        unsafe {
            match &self.mesh.as_ptr().as_ref().unwrap().trimesh.positions {
                Positions::F64(positions) => {
                    let colors = self
                        .mesh
                        .as_ptr()
                        .as_ref()
                        .unwrap()
                        .trimesh
                        .colors
                        .as_ref()
                        .unwrap();

                    let normals = self
                        .mesh
                        .as_ptr()
                        .as_ref()
                        .unwrap()
                        .trimesh
                        .normals
                        .as_ref()
                        .unwrap();

                    let meshref = MeshRef {
                        positions: &positions[start..end],
                        colors: &colors[start..end],
                        normals: &normals[start..end],
                        start,
                        end,
                    };

                    self.mesh.borrow_mut().child_models.last_mut().unwrap().main = Some(meshref);

                    Ok(())
                }
                _ => return Err(()),
            }
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

pub fn draw_cross_connection<'a>(
    center: &Vector3<f64>,
    start_cross: &Cross,
    end_cross: &Cross,
    color: &Srgba,
    coordinator: &'a PartCoordinator,
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
        cross.left + center,
        cross.right + center,
        cross.down + center,
        color,
        coordinator,
    )
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

#[derive(Debug)]
pub struct LayerMesh<'a> {
    pub trimesh: TriMesh,
    child_meshes: Vec<LayerPart<'a>>,
}

impl<'a> LayerMesh<'a> {
    pub fn empty() -> Self {
        Self {
            trimesh: TriMesh {
                positions: Positions::F64(Vec::new()),
                normals: Some(Vec::new()),
                colors: Some(Vec::new()),
                ..Default::default()
            },
            child_meshes: Vec::new(),
        }
    }

    pub fn new(main: TriMesh, child_meshes: Vec<LayerPart<'a>>) -> Self {
        Self {
            trimesh: main,
            child_meshes,
        }
    }
}

pub fn construct_filament_material() -> PhysicalMaterial {
    PhysicalMaterial {
        name: "default".to_string(),
        metallic: 0.0,
        roughness: 1.0,
        metallic_roughness_texture: None,
        normal_texture: None,
        normal_scale: 1.0,
        occlusion_texture: None,
        occlusion_strength: 1.0,
        render_states: RenderStates::default(),
        is_transparent: true,
        lighting_model: LightingModel::Phong,
        ..Default::default()
    }
}

pub struct LayerModel<'a> {
    pub model: Option<Gm<Mesh, PhysicalMaterial>>,
    pub trimesh: TriMesh,
    pub line_range: Option<(usize, usize)>,
    pub child_models: Vec<LayerPart<'a>>,
}

impl<'a> LayerModel<'a> {
    pub fn empty() -> Self {
        Self {
            model: None,
            trimesh: TriMesh {
                positions: Positions::F64(Vec::new()),
                normals: Some(Vec::new()),
                colors: Some(Vec::new()),
                ..Default::default()
            },
            line_range: None,
            child_models: Vec::new(),
        }
    }
}

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

pub struct MeshElements {
    positions: Vec<Vector3<f64>>,
    colors: Vec<Srgba>,
    normals: Vec<Vector3<f32>>,
}

impl MeshElements {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
            normals: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct MeshRef<'a> {
    pub positions: &'a [Vector3<f64>],
    colors: &'a [Srgba],
    normals: &'a [Vector3<f32>],
    start: usize,
    end: usize,
}
