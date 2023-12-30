use std::collections::HashMap;

use three_d::{vec3, Vector3};

use crate::{api::math::Average, model::shapes::VirtualBox, slicer::print_type::PrintType};

use super::{instruction::InstructionType, movement, state::State, GCode, WirePath};

#[derive(Debug, Clone)]
pub struct PathLine {
    pub start: Vector3<f32>,
    pub end: Vector3<f32>,
    pub print: bool,
}

impl PathLine {
    pub fn direction(&self) -> Vector3<f32> {
        self.end - self.start
    }
}

#[derive(Debug)]
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
}

#[derive(Debug, Default)]
pub struct RawPath {
    moduls: Vec<PathModul>,
    virtual_box: VirtualBox,
    center_mass: Option<Vector3<f32>>,
}

impl RawPath {
    pub fn moduls(&self) -> &Vec<PathModul> {
        &self.moduls
    }
}

impl From<&RawPath> for WirePath {
    fn from(raw: &RawPath) -> WirePath {
        let mut strokes = Vec::new();

        for path in raw.moduls {
            for line in path.paths {
                strokes.push(line);
            }
        }

        WirePath::new(strokes)
    }
}

impl From<GCode> for RawPath {
    fn from(value: GCode) -> Self {
        let mut moduls = Vec::new();

        let mut current_movements = movement::Movements::new();

        let mut toolpath_average = Average::<Vector3<f32>>::default();
        let mut virtual_box = VirtualBox::default();

        for instruction_modul in value.iter() {
            let mut strokes = Vec::new();

            //split instuctions into chunks of 5000 for performance
            let (modul_average, modul_box) =
                compute_modul(&mut strokes, &mut current_movements, instruction_modul);

            moduls.push(PathModul {
                paths: strokes.clone(),
                line_range: instruction_modul.range(),
                state: instruction_modul.state.clone(),
            });

            toolpath_average += modul_average;

            virtual_box.expand(modul_box);
        }

        let center_mass = if let Some(mut center) = toolpath_average.divide_average() {
            std::mem::swap(&mut center.y, &mut center.z);
            Some(center)
        } else {
            None
        };

        RawPath {
            moduls,
            virtual_box,
            center_mass,
        }
    }
}

fn compute_modul(
    points: &mut Vec<PathLine>,
    current_movements: &mut movement::Movements,
    instruction_modul: &super::instruction::InstructionModul,
) -> (Average<Vector3<f32>>, VirtualBox) {
    let mut modul_average = Average::<Vector3<f32>>::default();
    let mut virtual_box = VirtualBox::default();

    for instructions in instruction_modul.instructions().chunks(500) {
        let mut instruction_average = Average::<Vector3<f32>>::default();
        let mut instruction_box = VirtualBox::default();

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
                        let max = vec3(
                            current_point.x.max(last_point.x),
                            current_point.y.max(last_point.y),
                            current_point.z.max(last_point.z),
                        );
                        let min = vec3(
                            current_point.x.min(last_point.x),
                            current_point.y.min(last_point.y),
                            current_point.z.min(last_point.z),
                        );

                        let stroke_box = VirtualBox::new(max, min);

                        instruction_average.add((current_point + last_point) / 2.0);
                        instruction_box.expand(stroke_box);
                    }
                }
            }

            points.push(PathLine {
                start: last_point,
                end: current_point,
                print,
            });
        }

        modul_average += instruction_average;
        virtual_box.expand(instruction_box);
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
