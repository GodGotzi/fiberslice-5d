use core::panic;
use std::{
    collections::{hash_map::Iter, HashMap},
    path::Path,
    sync::Arc,
};

use rether::{
    alloc::{AllocHandle, DynamicAllocHandle},
    model::{geometry::Geometry, Model, TreeModel},
    picking::{interact::Interactive, Hitbox, HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer, Rotate, Scale, Translate,
};
use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};
use uni_path::PathBuf;

use crate::{geometry::BoundingBox, prelude::WgpuContext, GlobalState, RootEvent};

use super::gcode::{self, mesh::PathHitbox, DisplaySettings, MeshSettings, Toolpath, WireModel};

// const MAIN_LOADED_TOOLPATH: &str = "main"; // HACK: This is a solution to ease the dev when only one toolpath is loaded which is the only supported(for now)

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Load Error {0}")]
    LoadError(String),
    #[error("NoGeometryObject")]
    NoGeometryObject,
}

#[derive(Debug)]
pub struct ToolpathHandle {
    pub path: PathBuf,
    pub code: String,
    pub line_breaks: Vec<usize>,

    pub wire_model: WireModel,
    handle: Arc<TreeModel<Vertex, ToolpathContext, DynamicAllocHandle<Vertex>>>,
}

impl ToolpathHandle {
    pub fn code(&self) -> &String {
        &self.code
    }
}

pub enum ToolpathContext {
    Parent {
        box_: BoundingBox,
        enabled: bool,
    },
    Path {
        box_: Box<PathHitbox>,
        enabled: bool,
    },
}

impl std::fmt::Debug for ToolpathContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parent { box_, .. } => write!(f, "Parent({:?})", box_),
            Self::Path { box_, .. } => write!(f, "Path({:?})", box_),
        }
    }
}

impl ToolpathContext {
    pub fn parent(box_: BoundingBox) -> Self {
        Self::Parent {
            box_,
            enabled: true,
        }
    }

    pub fn path(box_: PathHitbox) -> Self {
        Self::Path {
            box_: Box::new(box_),
            enabled: true,
        }
    }
}

impl Translate for ToolpathContext {
    fn translate(&mut self, translation: glam::Vec3) {
        match self {
            Self::Parent { box_, .. } => box_.translate(translation),
            Self::Path { box_, .. } => box_.translate(translation),
        }
    }
}

impl Rotate for ToolpathContext {
    fn rotate(&mut self, rotation: glam::Quat) {
        match self {
            Self::Parent { box_, .. } => box_.rotate(rotation),
            Self::Path { box_, .. } => box_.rotate(rotation),
        };
    }
}

impl Scale for ToolpathContext {
    fn scale(&mut self, scale: glam::Vec3) {
        match self {
            Self::Parent { box_, .. } => box_.scale(scale),
            Self::Path { box_, .. } => box_.scale(scale),
        };
    }
}

impl Interactive for ToolpathContext {
    type Model = TreeModel<Vertex, ToolpathContext, DynamicAllocHandle<Vertex>>;

    fn clicked(
        &mut self,
        event: rether::picking::interact::ClickEvent,
    ) -> impl FnOnce(&Self::Model) {
        move |model| {
            println!("Clicked");
            println!("{:?}", event);
        }
    }

    fn scroll(
        &mut self,
        event: rether::picking::interact::ScrollEvent,
    ) -> impl FnOnce(&Self::Model) {
        move |model| {
            println!("Scrolled");
            println!("{:?}", event);
        }
    }

    fn drag(&mut self, event: rether::picking::interact::DragEvent) -> impl FnOnce(&Self::Model) {
        move |model| {
            println!("Dragged");
            println!("{:?}", event);
        }
    }
}

impl Hitbox for ToolpathContext {
    fn check_hit(&self, ray: &rether::picking::Ray) -> Option<f32> {
        match self {
            Self::Parent { box_, .. } => box_.check_hit(ray),
            Self::Path { box_, .. } => box_.check_hit(ray),
        }
    }

    fn expand_hitbox(&mut self, _box: &dyn Hitbox) {
        match self {
            Self::Parent { box_, .. } => box_.expand_hitbox(_box),
            Self::Path { box_, .. } => box_.expand_hitbox(_box),
        }
    }

    fn set_enabled(&mut self, enable: bool) {
        match self {
            Self::Parent { enabled, .. } => *enabled = enable,
            Self::Path { enabled, .. } => *enabled = enable,
        }
    }

    fn enabled(&self) -> bool {
        match self {
            Self::Parent { enabled, .. } => *enabled,
            Self::Path { enabled, .. } => *enabled,
        }
    }

    fn get_min(&self) -> glam::Vec3 {
        match self {
            Self::Parent { box_, .. } => box_.get_min(),
            Self::Path { box_, .. } => box_.get_min(),
        }
    }

    fn get_max(&self) -> glam::Vec3 {
        match self {
            Self::Parent { box_, .. } => box_.get_max(),
            Self::Path { box_, .. } => box_.get_max(),
        }
    }
}

// TODO also use vertex indices
#[derive(Debug)]
pub struct ToolpathServer {
    queue: Vec<(Receiver<Toolpath>, JoinHandle<()>)>,

    buffer: rether::Buffer<Vertex, rether::alloc::BufferDynamicAllocator<Vertex>>,

    root_hitbox: HitboxRoot<TreeModel<Vertex, ToolpathContext, DynamicAllocHandle<Vertex>>>,

    parts: HashMap<String, ToolpathHandle>,
    focused: Option<String>,
}

impl ToolpathServer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            queue: Vec::new(),
            buffer: Buffer::new("Toolpath Buffer", device),
            root_hitbox: HitboxRoot::root(),
            parts: HashMap::new(),
            focused: None,
        }
    }

    pub fn load<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let content = std::fs::read_to_string(&path).unwrap();
        let path = path.as_ref().to_str().unwrap_or("").to_string();
        let (tx, rx) = tokio::sync::oneshot::channel();

        let handle = tokio::spawn(async move {
            let mesh_settings = MeshSettings {};
            let display_settings = DisplaySettings {
                horizontal: 0.45,
                vertical: 0.325,
            };

            let gcode: gcode::GCode = gcode::parser::parse_content(&content).unwrap();

            let part = gcode::Toolpath::from_gcode(
                &path,
                (content.lines(), gcode),
                &mesh_settings,
                &display_settings,
            );

            tx.send(part).unwrap();
        });

        self.queue.push((rx, handle));
    }

    pub fn insert(
        &mut self,
        part: Toolpath,
        wgpu_context: &WgpuContext,
    ) -> Result<Arc<TreeModel<Vertex, ToolpathContext, DynamicAllocHandle<Vertex>>>, Error> {
        let path: PathBuf = part.origin_path.into();
        let file_name = if let Some(path) = path.file_name() {
            path.to_string()
        } else {
            path.to_string()
        };

        let mut name = file_name.clone();

        let mut counter: u8 = 1;

        while self.parts.contains_key(&name) {
            name = format!("{} ({counter})", file_name);

            counter += 1;
        }

        if let TreeModel::Root { state, .. } = &part.model {
            let handle = {
                let model_state = &*state.read();

                let data = match model_state {
                    rether::model::ModelState::Dormant(geometry) => geometry.build_data(),
                    _ => panic!("Unsupported geometry"),
                };

                self.buffer
                    .allocate_init(&name, data, &wgpu_context.device, &wgpu_context.queue)
            };

            part.model.wake(handle.clone());

            let code = part.raw.join("\n");

            let line_breaks = code
                .char_indices()
                .filter_map(|(index, c)| if c == '\n' { Some(index) } else { None })
                .collect::<Vec<usize>>();

            let handle = Arc::new(part.model);

            self.parts.insert(
                name.clone(),
                ToolpathHandle {
                    path,
                    code,
                    line_breaks,
                    wire_model: part.wire_model,
                    handle: handle.clone(),
                },
            );

            self.focused = Some(name.clone());

            Ok(handle.clone())
        } else {
            return Err(Error::NoGeometryObject);
        }
    }

    pub fn remove(&mut self, name: String, wgpu_context: &WgpuContext) {
        if let Some(part) = self.parts.remove(&name) {
            let state = part.handle.state();

            {
                let handle = match &*state.read() {
                    rether::model::ModelState::Awake(handle) => handle.clone(),
                    rether::model::ModelState::Destroyed => panic!("Already destroyed"),
                    _ => panic!("Not alive"),
                };

                self.buffer
                    .free(handle.id(), &wgpu_context.device, &wgpu_context.queue);
            }
        }
    }

    pub fn update(
        &mut self,
        global_state: GlobalState<RootEvent>,
        wgpu_context: &WgpuContext,
    ) -> Result<(), Error> {
        if !self.queue.is_empty() {
            let mut results = Vec::new();

            self.queue.retain_mut(|(rx, ..)| match rx.try_recv() {
                Ok(result) => {
                    results.push(result);

                    false
                }
                Err(TryRecvError::Closed) => false,
                _ => true,
            });

            for toolpath in results {
                let handle = self.insert(toolpath, wgpu_context)?;

                global_state
                    .ui_event_writer
                    .send(crate::ui::UiEvent::ShowSuccess("Gcode loaded".to_string()));

                global_state.camera_event_writer.send(
                    crate::camera::CameraEvent::UpdatePreferredDistance(BoundingBox::new(
                        handle.get_min(),
                        handle.get_max(),
                    )),
                );

                self.root_hitbox.add_node(handle);
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> Iter<'_, String, ToolpathHandle> {
        self.parts.iter()
    }

    pub fn iter_keys(&self) -> impl Iterator<Item = &String> {
        self.parts.keys()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut ToolpathHandle)> {
        self.parts.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn kill(&mut self) {
        for (_, handle) in self.queue.drain(..) {
            handle.abort();
        }
    }

    pub fn get_toolpath(&self, name: &str) -> Option<&ToolpathHandle> {
        self.parts.get(name)
    }

    pub fn get_toolpath_mut(&mut self, name: &str) -> Option<&mut ToolpathHandle> {
        self.parts.get_mut(name)
    }

    pub fn get_focused(&self) -> Option<&str> {
        self.focused.as_deref()
    }

    pub fn get_focused_mut(&mut self) -> &mut Option<String> {
        &mut self.focused
    }

    pub fn read_buffer(&self) -> &Buffer<Vertex, rether::alloc::BufferDynamicAllocator<Vertex>> {
        &self.buffer
    }
}
