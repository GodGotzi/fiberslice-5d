use std::collections::HashMap;

use glam::{vec3, Vec3};

use crate::{
    api::math::Average, geometry::BoundingHitbox, picking::hitbox::Hitbox,
    slicer::print_type::PrintType,
};

use super::{instruction::InstructionType, movement, state::PrintState, GCode};

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub print: bool,
}

impl Line {
    pub fn direction(&self) -> Vec3 {
        self.end - self.start
    }

    pub fn into_flipped_yz(&self) -> Line {
        Line {
            start: vec3(self.start.x, self.start.z, self.start.y),
            end: vec3(self.end.x, self.end.z, self.end.y),
            print: self.print,
        }
    }
}

#[derive(Debug)]
pub struct PathModul {
    pub lines: Vec<Line>,
    pub line_range: (usize, usize),
    pub state: PrintState,
}

impl PathModul {
    pub fn new(
        points: Vec<Line>,
        line_range: (usize, usize),
        state: super::state::PrintState,
    ) -> Self {
        Self {
            lines: points,
            line_range,
            state,
        }
    }
}

#[derive(Debug)]
pub struct RawPath {
    pub(super) moduls: Vec<PathModul>,
    pub(super) center_mass: Vec3,
    pub(super) virtual_box: BoundingHitbox,
}

impl From<&GCode> for RawPath {
    fn from(gcode: &GCode) -> Self {
        let mut moduls = Vec::new();

        let mut current_movements = movement::Movements::new();

        let mut toolpath_box = BoundingHitbox::default();
        let mut toolpath_average = Average::<Vec3>::default();

        for instruction_modul in gcode.iter() {
            let mut strokes = Vec::new();

            //split instuctions into chunks of 5000 for performance
            let (modul_average, virtual_box) =
                compute_modul(&mut strokes, &mut current_movements, instruction_modul);

            moduls.push(PathModul {
                lines: strokes.clone(),
                line_range: instruction_modul.range(),
                state: instruction_modul.state.clone(),
            });

            toolpath_average += modul_average;
            toolpath_box.expand(&virtual_box);
        }

        let center_mass = toolpath_average.divide_average();

        RawPath {
            moduls,
            center_mass: center_mass.unwrap_or(vec3(0.0, 0.0, 0.0)),
            virtual_box: toolpath_box,
        }
    }
}

fn compute_modul(
    points: &mut Vec<Line>,
    current_movements: &mut movement::Movements,
    instruction_modul: &super::instruction::InstructionModul,
) -> (Average<Vec3>, BoundingHitbox) {
    let mut modul_average = Average::<Vec3>::default();
    let mut virtual_box = BoundingHitbox::default();

    for instructions in instruction_modul.borrow_inner().chunks(500) {
        let mut instruction_average = Average::<Vec3>::default();

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

                virtual_box.expand_point(current_point);
            }

            points.push(Line {
                start: last_point,
                end: current_point,
                print,
            });
        }

        modul_average += instruction_average;
    }

    (modul_average, virtual_box)
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
