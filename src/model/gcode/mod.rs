use std::{collections::HashMap, fmt::Debug};

use three_d::{Gm, Mesh, PhysicalMaterial};

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    path::{PathLine, PathModul, RawPath},
    state::State,
};

use super::mesh::SimpleMesh;

pub mod instruction;
pub mod mesh;
pub mod movement;
pub mod parser;
pub mod path;
pub mod state;

pub type GCodeRaw = Vec<String>;
pub type GCode = Vec<InstructionModul>;

#[derive(Debug)]
pub struct ModulModel {
    mesh: SimpleMesh,
    line_range: (usize, usize),
    state: State,
}

pub type LayerModel = Vec<ModulModel>;

#[derive(Default)]
pub struct WorkpiecePath {
    layers: HashMap<usize, LayerModel>,
    gpu_model: Option<Gm<Mesh, PhysicalMaterial>>,
}

impl WorkpiecePath {
    pub fn from_gcode(gcode: &GCode) -> Self {
        let raw_path = RawPath::from(&gcode);

        let mut layers: HashMap<usize, LayerModel> = HashMap::new();

        {
            let modul_map: HashMap<usize, Vec<PathModul>> = raw_path.into();

            for (layer_id, moduls) in modul_map.into_iter() {
                let mut layer = Layer::empty();
                let mut coordinator = MeshCoordinator::new(&mut layer);

                for modul in moduls {
                    coordinator.compute_model(&modul);
                    coordinator.finish();
                }

                layers.insert(entry.0, layer);
            }
        }

        let mesh: three_d::CpuMesh = Layers(&layers).into();
    }
}

impl Debug for WorkpiecePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //only debug layers
        f.debug_struct("Path")
            .field("layers", &self.layers)
            .finish()
    }
}

pub struct WirePath {
    strokes: Vec<PathLine>,
}

impl WirePath {
    pub fn new(strokes: Vec<PathLine>) -> Self {
        Self { strokes }
    }
}

pub struct SourceBuilder {
    first: bool,
    source: String,
}

impl SourceBuilder {
    pub fn new() -> Self {
        Self {
            first: true,
            source: String::new(),
        }
    }

    pub fn push_movements(&mut self, movements: &Movements) {
        if let Some(x) = movements.X.as_ref() {
            self.push_movement("X", *x);
        }

        if let Some(y) = movements.Y.as_ref() {
            self.push_movement("Y", *y);
        }

        if let Some(z) = movements.Z.as_ref() {
            self.push_movement("Z", *z);
        }

        if let Some(e) = movements.E.as_ref() {
            self.push_movement("E", *e);
        }

        if let Some(f) = movements.F.as_ref() {
            self.push_movement("F", *f);
        }
    }

    pub fn push_movement(&mut self, movement_str: &str, value: f32) {
        if !self.first {
            self.source.push(' ');
        } else {
            self.first = false;
        }

        let code = format!("{}{}", movement_str, value);

        self.source.push_str(code.as_str());
    }

    pub fn push_instruction(&mut self, instruction: InstructionType) {
        if !self.first {
            self.source.push(' ');
        } else {
            self.first = false;
        }

        self.source.push_str(instruction.to_string().as_str());
    }

    pub fn finish(self) -> String {
        self.source
    }
}
