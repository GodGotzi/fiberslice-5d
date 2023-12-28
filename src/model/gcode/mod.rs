use std::{collections::HashMap, fmt::Debug};

use three_d::{CpuMesh, Gm, Mesh, PhysicalMaterial};

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    path::PathModul,
    state::State,
};

use super::{mesh::SimpleMesh, Model};

pub mod instruction;
pub mod mesh;
pub mod movement;
pub mod parser;
pub mod path;
pub mod state;

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

#[derive(Debug)]
pub struct ModulModel {
    mesh: SimpleMesh,
    line_range: (usize, usize),
    state: State,
}

pub type LayerModel = Vec<ModulModel>;

#[derive(Default)]
pub struct Path {
    layers: HashMap<usize, LayerModel>,
    gpu_model: Option<Gm<Mesh, PhysicalMaterial>>,
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //only debug layers
        f.debug_struct("Path")
            .field("layers", &self.layers)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct GCode {
    pub lines: Vec<String>,
    pub instruction_moduls: Vec<InstructionModul>,
}

impl GCode {
    pub fn new(lines: Vec<String>, moduls: Vec<InstructionModul>) -> Self {
        Self {
            lines,
            instruction_moduls: moduls,
        }
    }
}
