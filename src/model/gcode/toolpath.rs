use std::collections::HashMap;

use three_d::{vec3, Vector3};

use crate::{api::math::Average, slicer::print_type::PrintType};

use super::{
    instruction::InstructionType,
    mesh::{Layer, Layers, PartCoordinator},
    movement,
    state::State,
    GCode,
};

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

#[derive(Default)]
pub struct ToolPath {
    moduls: Vec<PathModul>,
    pub center: Option<Vector3<f32>>,
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
        let mut toolpath_average = Average::<Vector3<f32>>::default();

        for instruction_modul in value.instruction_moduls.iter() {
            let mut points = Vec::new();

            //split instuctions into chunks of 5000 for performance
            let modul_average =
                compute_instruction_modul(instruction_modul, &mut current_movements, &mut points);

            moduls.push(PathModul {
                paths: points.clone(),
                line_range: instruction_modul.range(),
                state: instruction_modul.state.clone(),
            });

            toolpath_average += modul_average;
        }

        let center = if let Some(mut center) = toolpath_average.divide_average() {
            std::mem::swap(&mut center.y, &mut center.z);
            Some(center)
        } else {
            None
        };

        ToolPath { moduls, center }
    }
}

fn compute_instruction_modul(
    instruction_modul: &super::instruction::InstructionModul,
    current_movements: &mut movement::Movements,
    points: &mut Vec<PathLine>,
) -> Average<Vector3<f32>> {
    let mut modul_average = Average::<Vector3<f32>>::default();

    for instructions in instruction_modul.instructions().chunks(500) {
        let mut instruction_average = Average::<Vector3<f32>>::default();

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
                if let Some(print_type) = instruction_modul.state.print_type.as_ref() {
                    if print_type == &PrintType::WallOuter
                        || print_type == &PrintType::ExternalPerimeter
                    {
                        instruction_average.add((current_point + last_point) / 2.0);
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
    }

    modul_average
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

#[derive(Debug)]
pub struct ToolpathModel {
    //pub layers: HashMap<usize, Layer>,
    pub gcode: GCode,
    pub center: Option<Vector3<f32>>,
}

impl GCode {
    pub fn into_mesh(self) -> (three_d::CpuMesh, ToolpathModel) {
        let toolpath = ToolPath::from(self.clone());
        let center = toolpath.center;

        let mut layers: HashMap<usize, Layer> = HashMap::new();

        {
            let modul_map: HashMap<usize, Vec<PathModul>> = toolpath.into();

            for entry in modul_map.into_iter() {
                let mut layer = Layer::empty();
                let mut coordinator = PartCoordinator::new(&mut layer);

                for modul in entry.1 {
                    coordinator.compute_model(&modul);
                    coordinator.finish();
                }

                layers.insert(entry.0, layer);
            }
        }

        let mesh: three_d::CpuMesh = Layers(&layers).into();

        (
            mesh,
            ToolpathModel {
                gcode: self,
                //layers,
                center,
            },
        )
    }
}
