use core::panic;
use std::{
    collections::{hash_map::Iter, HashMap},
    path::Path,
};

use rether::{
    alloc::BufferDynamicAllocator,
    picking::{Hitbox, HitboxNode},
    vertex::Vertex,
    Buffer, {IntoHandle, TreeHandle, TreeModel},
};
use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};
use uni_path::PathBuf;

use crate::{
    geometry::BoundingHitbox, picking::interact::InteractContext, prelude::WgpuContext,
    GlobalState, RootEvent,
};

use super::gcode::{self, DisplaySettings, MeshSettings, Toolpath, WireModel};

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
    handle: TreeHandle<InteractContext>,
}

impl ToolpathHandle {
    pub fn code(&self) -> &String {
        &self.code
    }
}

// TODO also use vertex indices
#[derive(Debug)]
pub struct ToolpathServer {
    queue: Vec<(Receiver<Toolpath>, JoinHandle<()>)>,

    buffer: rether::Buffer<Vertex, rether::alloc::BufferDynamicAllocator>,

    root_hitbox: HitboxNode<InteractContext>,

    parts: HashMap<String, ToolpathHandle>,
    focused: Option<String>,
}

impl ToolpathServer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            queue: Vec::new(),
            buffer: Buffer::new("Toolpath Buffer", device),
            root_hitbox: HitboxNode::root(),
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
    ) -> Result<TreeHandle<InteractContext>, Error> {
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

        if let TreeModel::Root { geometry, .. } = &part.model {
            self.buffer
                .allocate_init(&name, geometry, &wgpu_context.device, &wgpu_context.queue);
        } else {
            return Err(Error::NoGeometryObject);
        }

        let handle = part.model.req_handle(name.clone());

        let code = part.raw.join("\n");

        let line_breaks = code
            .char_indices()
            .filter_map(|(index, c)| if c == '\n' { Some(index) } else { None })
            .collect::<Vec<usize>>();

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

        self.focused = Some(name);

        Ok(handle)
    }

    pub fn remove(&mut self, name: String, wgpu_context: &WgpuContext) {
        if let Some(part) = self.parts.remove(&name) {
            match part.handle {
                TreeHandle::Root { id, .. } => {
                    self.buffer
                        .free(&id, &wgpu_context.device, &wgpu_context.queue);
                }
                _ => {
                    panic!("Why am I here?")
                }
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
                    crate::camera::CameraEvent::UpdatePreferredDistance(BoundingHitbox::new(
                        handle.min(),
                        handle.max(),
                    )),
                );

                self.root_hitbox.add_node(handle.into());
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

    pub fn read_buffer(&self) -> &Buffer<Vertex, BufferDynamicAllocator> {
        &self.buffer
    }

    pub fn root_hitbox(&self) -> &HitboxNode<InteractContext> {
        &self.root_hitbox
    }
}
