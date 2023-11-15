use std::collections::HashMap;

use bevy::{
    math::vec3,
    prelude::{Component, Mesh, Vec3},
};

use crate::slicer::print_type::PrintType;

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
    moduls: Vec<PathModul>,
    pub center: Option<Vec3>,
}

impl ToolPath {
    pub fn path(&self) -> &Vec<PathModul> {
        &self.moduls
    }
}

impl From<GCode> for ToolPath {
    fn from(value: GCode) -> Self {
        let mut moduls = Vec::new();

        let mut current_movements = movement::Movements::new();
        let mut toolpath_center = None;
        let mut modul_count = 0;

        for instruction_modul in value.instruction_moduls.iter() {
            let mut points = Vec::new();
            let mut modul_average = None;
            let mut chunk_count = 0;

            //split instuctions into chunks of 5000 for performance
            compute_instruction_modul(
                instruction_modul,
                &mut current_movements,
                &mut points,
                &mut chunk_count,
                &mut modul_average,
            );

            moduls.push(PathModul {
                paths: points.clone(),
                line_range: instruction_modul.range(),
                state: instruction_modul.state().clone(),
            });

            if let Some(average) = modul_average {
                modul_count += 1;

                if let Some(center) = toolpath_center.as_mut() {
                    *center += average / chunk_count as f32;
                } else {
                    toolpath_center = Some(average / chunk_count as f32);
                }
            }
        }

        if let Some(center) = toolpath_center.as_mut() {
            *center /= modul_count as f32;
            std::mem::swap(&mut center.y, &mut center.z)
        }

        ToolPath {
            moduls,
            center: toolpath_center,
        }
    }
}

fn compute_instruction_modul(
    instruction_modul: &super::instruction::InstructionModul,
    current_movements: &mut movement::Movements,
    points: &mut Vec<PathLine>,
    chunk_count: &mut i32,
    modul_average: &mut Option<Vec3>,
) {
    for instructions in instruction_modul.instructions().chunks(500) {
        let mut chunk_average = None;
        let mut instruction_count = 0;

        for instruction in instructions {
            let movements = instruction.movements();
            let last_point = current_movements.to_vec3(vec3(0.0, 0.0, 0.0));

            current_movements.add_movements(movements);

            let current_point = current_movements.to_vec3(vec3(0.0, 0.0, 0.0));

            if current_point == last_point {
                continue;
            }

            let print = instruction.instruction_type() == &InstructionType::G1
                && current_movements.E.is_some_and(|e| e > 0.0);

            if print {
                if let Some(print_type) = instruction_modul.state().print_type.as_ref() {
                    if print_type == &PrintType::WallOuter
                        || print_type == &PrintType::ExternalPerimeter
                    {
                        instruction_count += 1;

                        if let Some(average) = chunk_average.as_mut() {
                            *average += (current_point + last_point) / 2.0;
                        } else {
                            chunk_average = Some((current_point + last_point) / 2.0);
                        }
                    }
                }
            }

            points.push(PathLine {
                start: last_point,
                end: current_point,
                print,
            });
        }

        if let Some(chunk_average) = chunk_average {
            *chunk_count += 1;

            if let Some(average) = modul_average.as_mut() {
                *average += chunk_average / instruction_count as f32;
            } else {
                *modul_average = Some(chunk_average / instruction_count as f32);
            }
        }
    }
}

impl From<ToolPath> for HashMap<usize, Vec<PathModul>> {
    fn from(tool_path: ToolPath) -> Self {
        let mut layers: HashMap<usize, Vec<PathModul>> = HashMap::new();

        for path_modul in tool_path.moduls.into_iter() {
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
    pub center: Option<Vec3>,
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
