use std::{borrow::Borrow, collections::HashMap, fmt::Debug, str::Lines};

use glam::Vec3;

use crate::{
    picking::{hitbox::Hitbox, Pickable},
    prelude::{Shared, SharedMut},
    render::{buffer::BufferLocation, model::Model, vertex::Vertex},
};

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    path::{Line, RawPath},
    state::PrintState,
};

use crate::geometry::BoundingHitbox;

pub mod instruction;
pub mod mesh;
pub mod movement;
pub mod parser;
pub mod path;
pub mod state;

pub type GCodeRaw = Vec<String>;
pub type GCode = Vec<InstructionModul>;

#[derive(Debug, Clone)]
pub struct ModulModel {
    pub mesh: Vec<Vertex>,
    pub state: PrintState,
    range: (usize, usize),
}

pub type LayerModel = Vec<ModulModel>;

pub struct DisplaySettings {
    pub horizontal: f32,
    pub vertical: f32,
}

pub struct MeshSettings {}

#[derive(Debug, Clone)]
pub struct PrintPart {
    raw: GCodeRaw,
    wire_model: WireModel,

    pub model: Model<Vertex>,
    pub center_mass: Vec3,
    pub bounding_box: BoundingHitbox,
}

impl PrintPart {
    pub fn from_gcode(
        (raw, gcode): (Lines, GCode),
        mesh_settings: &MeshSettings,
        display_settings: &DisplaySettings,
    ) -> Self {
        let raw_path = RawPath::from(&gcode);

        let mut lines = Vec::new();

        let mut layers: HashMap<usize, LayerModel> = HashMap::new();

        let mut models = Vec::new();
        let mut bounding_box: Box<dyn Hitbox> = Box::<BoundingHitbox>::default();

        let mut vertices = Vec::new();

        for modul in raw_path.moduls {
            let layer = modul.state.layer.unwrap_or(0);
            let state = modul.state.clone();
            let range = modul.line_range;

            lines.extend(modul.lines.clone());

            let (modul_vertices, model) = modul.to_vertices(display_settings);

            bounding_box.expand(model.borrow().hitbox().unwrap());
            models.push(model);

            vertices.extend(modul_vertices);
        }

        for vertex in vertices.iter_mut() {
            vertex.position[0] -= raw_path.center_mass.x;
            vertex.position[1] -= raw_path.center_mass.y;
            vertex.position[2] -= raw_path.center_mass.z;
        }

        let box_ = BoundingHitbox::new(
            raw_path.virtual_box.min - raw_path.center_mass,
            raw_path.virtual_box.max - raw_path.center_mass,
        );

        let wire_model = WireModel::new(lines);

        let size = vertices.len();

        Self {
            raw: raw.map(|s| s.to_string()).collect(),
            wire_model,
            model: Model::Interactive {
                vertices,
                sub_meshes: models,
                location: BufferLocation { offset: 0, size },
                raw_box: SharedMut::from_inner(bounding_box),
                context: Shared::new(Box::new(TestContext {})),
            },

            center_mass: raw_path.center_mass,
            bounding_box: box_,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestContext {}

impl Pickable for TestContext {
    fn hover(&self, state: crate::GlobalState<crate::RootEvent>) {
        println!("Hovering")
    }

    fn select(&self, state: crate::GlobalState<crate::RootEvent>) {
        println!("Selecting")
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
