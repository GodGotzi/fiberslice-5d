use std::{fmt::Debug, str::Lines};

use glam::Vec3;
use rether::Translate;
use tree::ToolpathTree;

use crate::prelude::ArcModel;

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    path::{Line, RawPath},
};

pub mod instruction;
pub mod mesh;
pub mod movement;
pub mod parser;
pub mod path;
pub mod state;
pub mod tree;
pub mod vertex;

pub type GCodeRaw = Vec<String>;
pub type GCode = Vec<InstructionModul>;

pub struct DisplaySettings {
    pub horizontal: f32,
    pub vertical: f32,
}

pub struct MeshSettings {}

#[derive(Debug)]
pub struct Toolpath {
    pub origin_path: String,
    pub raw: GCodeRaw,
    pub wire_model: WireModel,
    pub model: ArcModel<ToolpathTree>,
    pub center_mass: Vec3,
}

unsafe impl Sync for Toolpath {}
unsafe impl Send for Toolpath {}

impl Toolpath {
    pub fn from_gcode(
        path: &str,
        (raw, gcode): (Lines, GCode),
        _mesh_settings: &MeshSettings,
        display_settings: &DisplaySettings,
    ) -> Self {
        let raw_path = RawPath::from(&gcode);

        let mut lines = Vec::new();

        // let mut layers: HashMap<usize, LayerModel> = HashMap::new();

        let mut root_vertices = Vec::new();
        let mut root = ToolpathTree::create_root();
        for modul in raw_path.moduls {
            lines.extend(modul.lines.clone());

            let (model, vertices) = modul.to_model(display_settings);

            root_vertices.extend(vertices);
            root.push_node(model);
        }
        root.awaken(&root_vertices);
        println!("{:?}", root_vertices.len());
        // println!("{:?}", root);
        drop(root_vertices);

        root.update_offset(0);
        root.translate(-raw_path.center_mass);

        let wire_model = WireModel::new(lines);

        Self {
            origin_path: path.to_string(),
            raw: raw.map(|s| s.to_string()).collect(),
            wire_model,
            model: ArcModel::new(root),
            center_mass: raw_path.center_mass,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WireModel {
    lines: Vec<Line>,
}

impl WireModel {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Line> {
        self.lines.iter()
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
