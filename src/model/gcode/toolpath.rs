use std::collections::HashMap;

use bevy::{
    math::vec3,
    prelude::{Component, Mesh, Vec3},
};

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
    pub paths: Vec<PathLine>,
    pub line_range: (usize, usize),
    pub state: State,
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

                if current_point == last_point {
                    continue;
                }

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

pub struct ToolPathModel {
    pub mesh: Mesh,
    pub layers: HashMap<usize, LayerContext>,
    pub gcode: GCode,
}

#[derive(Component)]
pub struct LayerContext {
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
