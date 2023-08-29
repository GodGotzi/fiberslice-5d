use std::collections::HashMap;

use three_d::Vector3;
use three_d_asset::{vec3, Positions, TriMesh};

use crate::model::mesh::*;

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

                draw_path((path.start, path.end), &color, &mut coordinator, &cross);
                last_cross = Some(cross);
            } else if let Some(last) = last_cross.take() {
                end_part(
                    (path.start, path.end),
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
                        (path.start, path.end),
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
            colors: Some(colors),
            ..Default::default()
        };

        mesh.compute_normals();

        Self::new(mesh, path_modul.state, path_modul.line_range, parts)
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
