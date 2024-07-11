use core::panic;
use std::{collections::HashMap, path::Path};

use tokio::{
    sync::oneshot::{error::TryRecvError, Receiver},
    task::JoinHandle,
};

use crate::{
    model::{IntoHandle, TreeHandle, TreeObject},
    picking::hitbox::PickContext,
    prelude::WgpuContext,
    render::{
        buffer::{alloc::BufferDynamicAllocator, DynamicBuffer},
        vertex::Vertex,
    },
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
    handle: TreeHandle<PickContext>,
}

#[derive(Debug)]
pub struct ToolpathServer {
    queue: Vec<(Receiver<Toolpath>, JoinHandle<()>)>,

    buffer: DynamicBuffer<Vertex, BufferDynamicAllocator>,
    parts: HashMap<String, ToolpathHandle>,
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

    pub fn insert(&mut self, part: Toolpath, wgpu_context: &WgpuContext) -> Result<(), Error> {
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

        self.parts.insert(name, ToolpathHandle { handle });

        Ok(())
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

    pub fn update(&mut self, wgpu_context: &WgpuContext) -> Result<(), Error> {
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
                self.insert(toolpath, wgpu_context)?;
            }
        }

        Ok(())
    }

    pub fn kill(&mut self) {
        for (_, handle) in self.queue.drain(..) {
            handle.abort();
        }
    }
}
