use std::collections::HashMap;

use three_d::Vector3;
use three_d_asset::{vec3, InnerSpace, Positions, Srgba, TriMesh};

use crate::model::mesh::{LayerMesh, LayerPartMesh, MeshGroup, PartCoordinator};

use super::{instruction::InstructionType, movement, state::State, GCode};

#[derive(Debug, Clone)]
pub struct PathLine {
    start: Vector3<f64>,
    end: Vector3<f64>,
    print: bool,
}

impl PathLine {
    pub fn direction(&self) -> Vector3<f64> {
        self.end - self.start
    }

    pub fn flip_yz(&self) -> Self {
        Self {
            start: vec3(self.start.x, self.start.z, self.start.y),
            end: vec3(self.end.x, self.end.z, self.end.y),
            print: self.print,
        }
    }
}

pub struct PathModul {
    paths: Vec<PathLine>,
    line_range: (usize, usize),
    state: State,
}

impl PathModul {
    pub fn new(
        points: Vec<PathLine>,
        line_range: (usize, usize),
        state: super::state::State,
    ) -> Self {
        Self {
            paths: points,
            line_range,
            state,
        }
    }

    pub fn points(&self) -> &Vec<PathLine> {
        &self.paths
    }

    pub fn state(&self) -> &super::state::State {
        &self.state
    }
}

#[derive(Default)]
pub struct ToolPath {
    path: Vec<PathModul>,
}

impl ToolPath {
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    pub fn add_line(
        &mut self,
        points: Vec<PathLine>,
        line_range: (usize, usize),
        state: super::state::State,
    ) {
        self.path.push(PathModul {
            paths: points,
            line_range,
            state,
        });
    }

    pub fn path(&self) -> &Vec<PathModul> {
        &self.path
    }
}

impl From<GCode> for ToolPath {
    fn from(value: GCode) -> Self {
        let mut tool_path = ToolPath::new();

        let mut current_movements = movement::Movements::new();

        for instruction_modul in value.instruction_moduls.iter() {
            let mut points = Vec::new();
            let state = instruction_modul.state();

            for instruction in instruction_modul.instructions() {
                let movements = instruction.movements();
                let last_point = current_movements.to_vec3(vec3(0.0, 0.0, 0.0));

                current_movements.add_movements(movements);

                let current_point = current_movements.to_vec3(vec3(0.0, 0.0, 0.0));

                let print = instruction.instruction_type() == &InstructionType::G1
                    && current_movements.E.is_some_and(|e| e > 0.0);

                points.push(PathLine {
                    start: last_point,
                    end: current_point,
                    print,
                });
            }

            tool_path.add_line(points, instruction_modul.range(), state.clone());
        }

        tool_path
    }
}

#[allow(dead_code)]
impl From<PathModul> for LayerPartMesh {
    fn from(path_modul: PathModul) -> Self {
        let diameter = 0.4;
        let mut last_cross: Option<Cross> = None;

        let color = path_modul
            .state
            .print_type
            .as_ref()
            .unwrap_or(&crate::slicer::print_type::PrintType::Unknown)
            .get_color();

        let mut positions = Vec::new();
        let mut colors = Vec::new();

        let mut coordinator = PartCoordinator::new();
        let mut parts = Vec::new();

        for element in path_modul.paths.iter().enumerate() {
            let path = element.1;

            if path.print {
                let direction = path.direction();
                let cross = get_cross(direction, diameter / 2.0);

                if let Some(last) = last_cross.take() {
                    draw_cross_connection(&path.start, &cross, &last, &color, &mut coordinator);
                } else {
                    draw_rect_with_cross(&path.start, &cross, &color, &mut coordinator);
                }

                draw_path(path, &color, &mut coordinator, &cross);
                last_cross = Some(cross);
            } else if let Some(last) = last_cross.take() {
                end_part(
                    path,
                    &color,
                    last,
                    &mut coordinator,
                    &mut positions,
                    &mut colors,
                    &mut parts,
                );
            }

            if element.0 == path_modul.paths.len() - 1 {
                if let Some(last) = last_cross.take() {
                    end_part(
                        path,
                        &color,
                        last,
                        &mut coordinator,
                        &mut positions,
                        &mut colors,
                        &mut parts,
                    );
                }
            }
        }

        let mut mesh = TriMesh {
            positions: Positions::F64(positions),
            ..Default::default()
        };

        mesh.compute_normals();

        Self::new(mesh, path_modul.state, path_modul.line_range, parts)
    }
}

fn end_part(
    path: &PathLine,
    color: &Srgba,
    last: Cross,
    coordinator: &mut PartCoordinator,
    positions: &mut Vec<Vector3<f64>>,
    colors: &mut Vec<Srgba>,
    layer_parts: &mut Vec<TriMesh>,
) {
    draw_rect_with_cross(&path.start, &last, color, coordinator);

    let next_mesh = coordinator.next_trimesh();

    //end part
    positions.extend(next_mesh.positions.to_f64().iter());
    colors.extend(next_mesh.colors.as_ref().unwrap().iter());

    layer_parts.push(next_mesh);
}

fn draw_path(path: &PathLine, color: &Srgba, coordinator: &mut PartCoordinator, cross: &Cross) {
    draw_rect(
        cross.up + path.start,
        cross.right + path.start,
        cross.up + path.end,
        cross.right + path.end,
        color,
        coordinator,
    );

    draw_rect(
        cross.down + path.start,
        cross.right + path.start,
        cross.down + path.end,
        cross.right + path.end,
        color,
        coordinator,
    );

    draw_rect(
        cross.down + path.start,
        cross.left + path.start,
        cross.down + path.end,
        cross.left + path.end,
        color,
        coordinator,
    );

    draw_rect(
        cross.up + path.start,
        cross.left + path.start,
        cross.up + path.end,
        cross.left + path.end,
        color,
        coordinator,
    );
}

fn draw_cross_connection(
    center: &Vector3<f64>,
    start_cross: &Cross,
    end_cross: &Cross,
    color: &Srgba,
    coordinator: &mut PartCoordinator,
) {
    //top
    coordinator.add_position(end_cross.up + center);
    coordinator.add_position(end_cross.right + center);
    coordinator.add_position(start_cross.right + center);

    coordinator.add_color(*color);

    coordinator.add_position(end_cross.up + center);
    coordinator.add_position(end_cross.left + center);
    coordinator.add_position(start_cross.left + center);

    coordinator.add_color(*color);

    //bottom
    coordinator.add_position(end_cross.down + center);
    coordinator.add_position(end_cross.right + center);
    coordinator.add_position(start_cross.right + center);

    coordinator.add_color(*color);

    coordinator.add_position(end_cross.down + center);
    coordinator.add_position(end_cross.left + center);
    coordinator.add_position(start_cross.left + center);

    coordinator.add_color(*color);
}

fn draw_rect(
    point_left_0: Vector3<f64>,
    point_left_1: Vector3<f64>,
    point_right_0: Vector3<f64>,
    point_right_1: Vector3<f64>,
    color: &Srgba,
    coordinator: &mut PartCoordinator,
) {
    coordinator.add_position(point_left_0);
    coordinator.add_position(point_left_1);
    coordinator.add_position(point_right_0);

    coordinator.add_color(*color);

    coordinator.add_position(point_right_0);
    coordinator.add_position(point_right_1);
    coordinator.add_position(point_left_1);

    coordinator.add_color(*color);
}

fn draw_rect_with_cross(
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
struct Cross {
    up: Vector3<f64>,
    down: Vector3<f64>,
    left: Vector3<f64>,
    right: Vector3<f64>,
}

fn get_cross(direction: Vector3<f64>, radius: f64) -> Cross {
    let horizontal = direction.cross(vec3(0.0, 0.0, direction.z + 1.0));
    let vertical = direction.cross(vec3(direction.x + 1.0, direction.y + 1.0, 0.0));

    Cross {
        up: vertical.normalize() * radius,
        down: vertical.normalize() * (-radius),
        left: horizontal.normalize() * radius,
        right: horizontal.normalize() * (-radius),
    }
}

impl From<ToolPath> for Vec<LayerMesh> {
    fn from(tool_path: ToolPath) -> Self {
        let mut triangles = 0;

        let mut layers: HashMap<usize, Vec<LayerPartMesh>> = HashMap::new();

        for path_modul in tool_path.path.into_iter() {
            let state = path_modul.state.clone();
            let mesh = LayerPartMesh::from(path_modul);

            if let Some(layer) = layers.get_mut(&state.layer.unwrap_or(0)) {
                layer.push(mesh);
            } else {
                layers.insert(state.layer.unwrap_or(0), vec![mesh]);
            }
        }

        println!("Triangles: {}", triangles);

        layers
            .into_values()
            .map(|v| {
                let mesh: LayerMesh = v.into();
                triangles += mesh.tri_count();
                mesh
            })
            .collect()
    }
}
