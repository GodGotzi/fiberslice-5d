use std::{collections::HashMap, fmt::Debug, str::Lines};

use three_d::{Gm, Mesh, PhysicalMaterial, Vector3};

use crate::settings::{DisplaySettings, Settings};

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    path::{PathStroke, RawPath},
    state::State,
};

use super::{mesh::Vertices, shapes::VirtualBox};

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
    mesh: Vertices,
    child_offsets: Vec<usize>,
    state: State,
    range: (usize, usize),
}

pub type LayerModel = Vec<ModulModel>;

pub struct WorkpiecePath {
    raw: GCodeRaw,
    wire_model: WirePath,
    layers: HashMap<usize, LayerModel>,
    virtual_box: VirtualBox,
    center_mass: Vector3<f32>,
}

impl WorkpiecePath {
    pub fn from_gcode((raw, gcode): (Lines, GCode), settings: &Settings) -> Self {
        let raw_path = RawPath::from(&gcode);

        let mut strokes = Vec::new();

        let mut layers: HashMap<usize, LayerModel> = HashMap::new();

        for modul in raw_path.moduls {
            let layer = modul.state.layer.unwrap_or(0);
            let state = modul.state.clone();
            let range = modul.line_range;

            strokes.extend(modul.paths.clone());

            let (vertices, child_offsets) = modul.to_vertices(settings);

            let model = ModulModel {
                mesh: vertices,
                child_offsets,
                state,
                range,
            };

            layers.entry(layer).or_default().push(model);
        }

        let wire_model = WirePath::new(strokes);

        Self {
            raw: raw.map(|s| s.to_string()).collect(),
            wire_model,
            layers,
            virtual_box: raw_path.virtual_box,
            center_mass: raw_path.center_mass,
        }
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
    strokes: Vec<PathStroke>,
}

impl WirePath {
    pub fn new(strokes: Vec<PathStroke>) -> Self {
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
