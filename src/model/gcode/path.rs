use std::collections::HashMap;

use three_d::{vec3, Vector3};

use crate::{api::math::Average, model::shapes::VirtualBox, slicer::print_type::PrintType};

use super::{instruction::InstructionType, movement, state::State, GCode};

#[derive(Debug, Clone)]
pub struct PathStroke {
    pub start: Vector3<f32>,
    pub end: Vector3<f32>,
    pub print: bool,
}

impl PathStroke {
    pub fn direction(&self) -> Vector3<f32> {
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct PathModul {
    pub paths: Vec<PathStroke>,
    pub line_range: (usize, usize),
    pub state: State,
}

impl PathModul {
    pub fn new(
        points: Vec<PathStroke>,
        line_range: (usize, usize),
        state: super::state::State,
    ) -> Self {
        Self {
            paths: points,
            line_range,
            state,
        }
    }
}

#[derive(Debug)]
pub struct RawPath {
    pub(super) moduls: Vec<PathModul>,
    pub(super) center_mass: Vector3<f32>,
}

impl From<&GCode> for RawPath {
    fn from(gcode: &GCode) -> Self {
        let mut moduls = Vec::new();

        let mut current_movements = movement::Movements::new();

        let mut toolpath_average = Average::<Vector3<f32>>::default();

        for instruction_modul in gcode.iter() {
            let mut strokes = Vec::new();

            //split instuctions into chunks of 5000 for performance
            let modul_average =
                compute_modul(&mut strokes, &mut current_movements, instruction_modul);

            moduls.push(PathModul {
                paths: strokes.clone(),
                line_range: instruction_modul.range(),
                state: instruction_modul.state.clone(),
            });

            toolpath_average += modul_average;
        }

        let center_mass = if let Some(mut center) = toolpath_average.divide_average() {
            std::mem::swap(&mut center.y, &mut center.z);
            Some(center)
        } else {
            None
        };

        RawPath {
            moduls,
            center_mass: center_mass.unwrap_or(vec3(0.0, 0.0, 0.0)),
        }
    }
}

fn compute_modul(
    points: &mut Vec<PathStroke>,
    current_movements: &mut movement::Movements,
    instruction_modul: &super::instruction::InstructionModul,
) -> Average<Vector3<f32>> {
    let mut modul_average = Average::<Vector3<f32>>::default();

    for instructions in instruction_modul.borrow_inner().chunks(500) {
        let mut instruction_average = Average::<Vector3<f32>>::default();

        for instruction in instructions {
            let movements = instruction.movements();
            let last_point = current_movements.to_flipped_vec3(vec3(0.0, 0.0, 0.0));

            current_movements.add_movements(movements);

            let current_point = current_movements.to_flipped_vec3(vec3(0.0, 0.0, 0.0));

            if current_point == last_point {
                continue;
            }

            let print = instruction.instruction_type() == &InstructionType::G1
                && current_movements.E.is_some_and(|e| e > 0.0);

            if print {
                if let Some(print_type) = instruction_modul.state.print_type.as_ref() {
                    if print_type == &PrintType::WallOuter
                        || print_type == &PrintType::ExternalPerimeter
                    {
                        instruction_average.add((current_point + last_point) / 2.0);
                    }
                }
            }

            points.push(PathStroke {
                start: last_point,
                end: current_point,
                print,
            });
        }

        modul_average += instruction_average;
    }

    modul_average
}

impl From<RawPath> for HashMap<usize, Vec<PathModul>> {
    fn from(tool_path: RawPath) -> Self {
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
