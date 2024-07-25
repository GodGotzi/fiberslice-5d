use std::{fmt::Debug, str::Lines};

use glam::{vec4, Vec3};

use crate::{
    geometry::{
        mesh::{Mesh, WireMesh},
        SelectBox,
    },
    model::{
        transform::{Rotate, Scale, Translate},
        TreeObject,
    },
    picking::{
        hitbox::{Hitbox, PickContext},
        interactive::Pickable,
    },
    prelude::{SharedMut, WgpuContext},
    render::vertex::Vertex,
};

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    path::{Line, RawPath},
    state::PrintState,
};

use crate::geometry::BoundingHitbox;

use super::ToVisual;

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
pub struct Toolpath {
    pub origin_path: String,
    pub raw: GCodeRaw,
    pub wire_model: WireModel,
    pub model: TreeObject<Vertex, SharedMut<Box<dyn Pickable>>>,
    pub center_mass: Vec3,
}

impl Toolpath {
    pub fn from_gcode(
        path: &str,
        (raw, gcode): (Lines, GCode),
        mesh_settings: &MeshSettings,
        display_settings: &DisplaySettings,
    ) -> Self {
        let raw_path = RawPath::from(&gcode);

        let mut lines = Vec::new();

        // let mut layers: HashMap<usize, LayerModel> = HashMap::new();

        let mut root: TreeObject<Vertex, PickContext> = TreeObject::Root {
            geometry: Vec::new(),
            sub_models: Vec::new(),
            ctx: SharedMut::from_inner(Box::new(PathContext {
                box_: BoundingHitbox::default(),
            })),
        };

        for modul in raw_path.moduls {
            lines.extend(modul.lines.clone());

            let (modul_vertices, model) = modul.to_vertices(display_settings);

            root.expand(model);
            root.extend_data(modul_vertices);
        }

        root.translate(-raw_path.center_mass);

        let wire_model = WireModel::new(lines);

        Self {
            origin_path: path.to_string(),
            raw: raw.map(|s| s.to_string()).collect(),
            wire_model,
            model: root,
            center_mass: raw_path.center_mass,
        }
    }
}

#[derive(Debug)]
pub struct PathContext<T> {
    box_: T,
}

impl<T: Translate> Translate for PathContext<T> {
    fn translate(&mut self, translation: Vec3) {
        self.box_.translate(translation)
    }
}

impl<T: Rotate> Rotate for PathContext<T> {
    fn rotate(&mut self, rotation: glam::Quat) {
        self.box_.rotate(rotation)
    }
}

impl<T: Scale> Scale for PathContext<T> {
    fn scale(&mut self, scale: Vec3) {
        self.box_.scale(scale)
    }
}

impl<T: Hitbox> Hitbox for PathContext<T> {
    fn check_hit(&self, ray: &crate::picking::ray::Ray) -> Option<f32> {
        self.box_.check_hit(ray)
    }

    fn expand(&mut self, _box: &dyn Hitbox) {
        self.box_.expand(_box)
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.box_.set_enabled(enabled)
    }

    fn enabled(&self) -> bool {
        self.box_.enabled()
    }

    fn min(&self) -> Vec3 {
        self.box_.min()
    }

    fn max(&self) -> Vec3 {
        self.box_.max()
    }
}

impl<T: Hitbox + ToVisual<72, 48>> Pickable for PathContext<T> {
    fn picked(
        &self,
        global_state: &crate::GlobalState<crate::RootEvent>,
        wgpu_context: &WgpuContext,
    ) {
        let diagonal = self.max() - self.min();
        let distance = diagonal.x.min(diagonal.y).min(diagonal.z);

        let select_box: SelectBox = SelectBox::from(BoundingHitbox::new(
            self.min() - distance,
            self.max() + distance,
        ))
        .with_color(vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 1.0, 1.0));

        let select_smaller_box: SelectBox = SelectBox::from(BoundingHitbox::new(
            self.min() - distance * 0.1,
            self.max() + distance * 0.1,
        ))
        .with_color(vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 1.0, 1.0));

        let visual = self.box_.to_visual();

        global_state.widget_test_buffer.write().write(
            &wgpu_context.queue,
            "select_box",
            &visual.vertices,
        );

        global_state.widget_wire_test_buffer.write().write(
            &wgpu_context.queue,
            "select_box",
            &visual.wires,
        );
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
