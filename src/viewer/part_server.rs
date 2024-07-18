use core::panic;
use std::{collections::HashMap, path::Path};

use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};

use crate::{
    geometry::BoundingHitbox,
    model::{IntoHandle, TreeHandle, TreeObject},
    picking::{
        hitbox::{Hitbox, PickContext},
        interactive::Pickable,
    },
    prelude::WgpuContext,
    render::{
        buffer::{alloc::BufferDynamicAllocator, DynamicBuffer},
        vertex::Vertex,
    },
    GlobalState, RootEvent,
};

use super::gcode::{self, DisplaySettings, MeshSettings, Toolpath};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Load Error {0}")]
    LoadError(String),
    #[error("NoGeometryObject")]
    NoGeometryObject,
}

#[derive(Debug)]
pub struct ToolpathHandle {
    pub code: String,
    pub line_breaks: Vec<usize>,
    handle: TreeHandle<PickContext>,
}

impl ToolpathHandle {
    pub fn code(&self) -> &String {
        &self.code
    }
}

#[derive(Debug)]
pub struct ToolpathServer {
    queue: Vec<(Receiver<Toolpath>, JoinHandle<()>)>,

    buffer: DynamicBuffer<Vertex, BufferDynamicAllocator>,
    parts: HashMap<String, ToolpathHandle>,
    focused: Option<String>,
}

impl ToolpathServer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            queue: Vec::new(),
            buffer: DynamicBuffer::new(
                BufferDynamicAllocator::default(),
                "Toolpath Buffer",
                device,
            ),
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
    ) -> Result<TreeHandle<crate::prelude::SharedMut<Box<dyn Pickable>>>, Error> {
        let mut name = part.origin_path.to_string();
        let mut counter: u8 = 1;

        while self.parts.contains_key(&name) {
            name = format!("{} ({counter})", part.origin_path);

            counter += 1;
        }

        if let TreeObject::Root { geometry, .. } = &part.model {
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
                code,
                line_breaks,
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

                global_state.picking_state.add_hitbox(handle.into());
            }
        }

        Ok(())
    }

    pub fn kill(&mut self) {
        for (_, handle) in self.queue.drain(..) {
            handle.abort();
        }
    }

    pub fn get_focused(&self) -> Option<&ToolpathHandle> {
        if let Some(focused_toolpath) = self
            .parts
            .get(self.focused.as_ref().unwrap_or(&"".to_string()))
        {
            Some(focused_toolpath)
        } else {
            None
        }
    }

    pub fn read_buffer(&self) -> &DynamicBuffer<Vertex, BufferDynamicAllocator> {
        &self.buffer
    }
}
