use three_d::{Gm, Mesh, PhysicalMaterial, RenderStates, WindowedContext};
use three_d_asset::{vec3, InnerSpace, LightingModel, Positions, Srgba, TriMesh, Vector3};

use super::gcode::state::State;

pub struct PartCoordinator<'a> {
    parts: &'a mut MeshElements,
    offset_start: usize,
    offset_end: usize,
    offset_part_start: usize,
    offset_part_end: usize,
}

impl<'a> PartCoordinator<'a> {
    pub fn new(parts: &'a mut MeshElements) -> Self {
        Self {
            parts,
            offset_start: 0,
            offset_end: 0,
            offset_part_start: 0,
            offset_part_end: 0,
        }
    }

    pub fn add_triangle(
        &mut self,
        triangle: (Vector3<f64>, Vector3<f64>, Vector3<f64>),
        color: Srgba,
    ) {
        self.parts.positions.push(triangle.0);
        self.parts.positions.push(triangle.1);
        self.parts.positions.push(triangle.2);

        self.parts.colors.push(color);
        self.parts.colors.push(color);
        self.parts.colors.push(color);

        let normal_f64 = (triangle.1 - triangle.0)
            .cross(triangle.2 - triangle.0)
            .normalize();

        let normal = Vector3::new(
            normal_f64.x as f32,
            normal_f64.y as f32,
            normal_f64.z as f32,
        );

        self.parts.normals.push(normal);
        self.parts.normals.push(normal);
        self.parts.normals.push(normal);

        self.offset_end += 3;
        self.offset_part_end += 3;
    }

    pub fn next_part_meshref(&mut self) -> MeshRef<'_> {
        let start = self.offset_part_start;
        let end = self.offset_part_end;

        self.offset_part_start = end;

        MeshRef {
            positions: &mut self.parts.positions[start..end],
            colors: &mut self.parts.colors[start..end],
            normals: &mut self.parts.normals[start..end],
        }
    }

    pub fn finish(&mut self) -> MeshRef<'_> {
        let start = self.offset_start;
        let end = self.offset_end;

        self.offset_start = end;

        MeshRef {
            positions: &mut self.parts.positions[start..end],
            colors: &mut self.parts.colors[start..end],
            normals: &mut self.parts.normals[start..end],
        }
    }
}

pub fn draw_path(
    path: (Vector3<f64>, Vector3<f64>),
    color: &Srgba,
    coordinator: &mut PartCoordinator,
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
    coordinator: &'a mut PartCoordinator<'a>,
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
    coordinator: &mut PartCoordinator,
) {
    coordinator.add_triangle((point_left_0, point_left_1, point_right_0), *color);

    coordinator.add_triangle((point_right_0, point_right_1, point_left_1), *color);
}

pub fn draw_rect_with_cross(
    center: &Vector3<f64>,
    cross: &Cross,
    color: &Srgba,
    coordinator: &mut PartCoordinator,
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
    pub fn new(main: TriMesh, child_meshes: Vec<LayerPart<'a>>) -> Self {
        Self {
            trimesh: main,
            child_meshes,
        }
    }
}

impl<'a> LayerMesh<'a> {
    fn from_with_mesh_elements(mesh_elements: MeshElements, parts: Vec<LayerPart<'a>>) -> Self {
        let mesh = TriMesh {
            positions: Positions::F64(mesh_elements.positions.to_owned()),
            colors: Some(mesh_elements.colors.to_owned()),
            normals: Some(mesh_elements.normals.to_owned()),
            ..Default::default()
        };

        Self {
            trimesh: mesh,
            child_meshes: parts,
        }
    }
}

impl<'a> LayerMesh<'a> {
    pub fn into_model(self, context: &WindowedContext) -> LayerModel<'a> {
        let model = Gm::new(
            Mesh::new(context, &self.trimesh),
            construct_filament_material(),
        );

        let mut min_line = usize::MAX;
        let mut max_line = usize::MIN;

        for part in &self.child_meshes {
            min_line = min_line.min(part.line_range.0);
            max_line = max_line.max(part.line_range.1);
        }

        //let slice = &child_models[0..5];

        LayerModel {
            model,
            line_range: (0, 0),
            child_models: self.child_meshes,
        }
    }

    pub fn tri_count(&self) -> usize {
        self.trimesh.positions.len() / 3
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
    pub model: Gm<Mesh, PhysicalMaterial>,
    pub line_range: (usize, usize),
    pub child_models: Vec<LayerPart<'a>>,
}

#[derive(Debug)]
pub struct LayerPart<'a> {
    main: MeshRef<'a>,
    state: State,
    line_range: (usize, usize),
    child_meshes: Vec<MeshRef<'a>>,
}

impl<'a> LayerPart<'a> {
    pub fn new(
        main: MeshRef<'a>,
        state: State,
        line_range: (usize, usize),
        child_meshes: Vec<MeshRef<'a>>,
    ) -> Self {
        Self {
            main,
            state,
            line_range,
            child_meshes,
        }
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
    positions: &'a [Vector3<f64>],
    colors: &'a [Srgba],
    normals: &'a [Vector3<f32>],
}
