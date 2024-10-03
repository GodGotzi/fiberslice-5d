use core::panic;
use std::{
    collections::{hash_map::Iter, HashMap},
    path::Path,
    sync::Arc,
};

use rether::{
    alloc::DynamicAllocHandle,
    model::{geometry::Geometry, Model},
    picking::{HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer,
};

use rether::alloc::AllocHandle;
use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};
use uni_path::PathBuf;

use crate::{
    geometry::BoundingBox, prelude::WgpuContext, viewer::toolpath::pipeline::ToolpathBuffer,
    GlobalState, RootEvent,
};

use crate::viewer::toolpath::{
    self, tree::ToolpathTree, DisplaySettings, MeshSettings, Toolpath, WireModel,
};

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
    pub handle: Arc<ToolpathTree>,
}

impl ToolpathHandle {
    pub fn code(&self) -> &String {
        &self.code
    }
}

// TODO also use vertex indices
#[derive(Debug)]
pub struct ToolpathServer {
    queue: Option<(Receiver<Toolpath>, JoinHandle<()>)>,

    buffer: ToolpathBuffer,

    root_hitbox: HitboxRoot<ToolpathTree>,

    parts: HashMap<String, ToolpathHandle>,
    focused: Option<String>,
}

impl ToolpathServer {
    pub fn new(
        context: &WgpuContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            queue: None,
            buffer: ToolpathBuffer::new(context, camera_bind_group_layout, light_bind_group_layout),
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

            let gcode: toolpath::GCode = toolpath::parser::parse_content(&content).unwrap();

            let part = toolpath::Toolpath::from_gcode(
                &path,
                (content.lines(), gcode),
                &mesh_settings,
                &display_settings,
            );

            tx.send(part).unwrap();
        });

        self.queue = Some((rx, handle));
    }

    pub fn insert(
        &mut self,
        part: Toolpath,
        wgpu_context: &WgpuContext,
    ) -> Result<Arc<ToolpathTree>, Error> {
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

        let handle = {
            let model_state = &*part.model.state().read();

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
        if let Some((rx, _)) = &self.queue {
            let mut results = Vec::new();

            if let Ok(toolpath) = rx.try_recv() {
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

        self.buffer
            .update(&wgpu_context.device, &wgpu_context.queue);

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

    pub fn root_hitbox(&self) -> &HitboxRoot<ToolpathTree> {
        &self.root_hitbox
    }
}
