use std::{cell::RefCell, collections::HashMap};

use bevy::{
    math::vec3,
    prelude::{Component, Mesh, Vec3},
};

use crate::model::layer::*;

use super::{instruction::InstructionType, movement, state::State, GCode};

#[derive(Debug, Clone)]
pub struct PathLine {
    pub start: Vec3,
    pub end: Vec3,
    pub print: bool,
}

impl PathLine {
    pub fn direction(&self) -> Vec3 {
        self.end - self.start
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

pub fn compute_modul_with_coordinator<'a>(
    path_modul: &'a PathModul,
    coordinator: &'a PartCoordinator,
) {
    let diameter = 0.05;
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

        if path.print {
            let direction = path.direction();
            let cross = get_cross(direction, diameter / 2.0);

            if let Some(last) = last_cross.take() {
                draw_cross_connection(&path.start, &cross, &last, &color, coordinator);
            } else {
                draw_rect_with_cross(&path.start, &cross, &color, coordinator);
            }

            draw_path((path.start, path.end), &color, coordinator, &cross);
            last_cross = Some(cross);
        } else if let Some(last) = last_cross.take() {
            draw_rect_with_cross(&path.end, &last, &color, coordinator);

            coordinator
                .next_part_meshref(path_modul.state.clone(), path_modul.line_range)
                .unwrap();
        }

        if element.0 == path_modul.paths.len() - 1 {
            if let Some(last) = last_cross.take() {
                draw_rect_with_cross(&path.end, &last, &color, coordinator);

                coordinator
                    .next_part_meshref(path_modul.state.clone(), path_modul.line_range)
                    .unwrap();
            }
        }
    }

    coordinator.finish().unwrap();
}

impl From<ToolPath> for HashMap<usize, Vec<PathModul>> {
    fn from(tool_path: ToolPath) -> Self {
        let mut layers: HashMap<usize, Vec<PathModul>> = HashMap::new();

        for path_modul in tool_path.path.into_iter() {
            let layer = path_modul.state.layer.unwrap_or(0);

            if let Some(layer_moduls) = layers.get_mut(&layer) {
                layer_moduls.push(path_modul);
            } else {
                layers.insert(layer, vec![path_modul]);
            }
        }

        layers
    }
}

pub struct SyncableLayers<'a>(HashMap<usize, LayerMesh<'a>>);

impl<'a> From<HashMap<usize, RefCell<LayerMesh<'a>>>> for SyncableLayers<'a> {
    fn from(value: HashMap<usize, RefCell<LayerMesh<'a>>>) -> Self {
        let mut layers = HashMap::new();

        for entry in value.into_iter() {
            layers.insert(entry.0, entry.1.into_inner());
        }

        Self(layers)
    }
}

pub struct ToolPathModel {
    pub mesh: Mesh,
    pub layers: HashMap<usize, MeshModel>,
    pub gcode: GCode,
}

pub struct MeshModel {
    pub line_range: Option<(usize, usize)>,
}

#[derive(Component)]
pub struct Layer {
    pub id: usize,
    pub line_range: Option<(usize, usize)>,
}

impl std::fmt::Debug for ToolPathModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolPathModel")
            .field("layers", &self.layers.keys())
            .finish()
    }
}
