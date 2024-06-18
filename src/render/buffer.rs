use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor,
};

use super::{mesh::MeshKit, vertex::Vertex};

const MAX_WIDGETS_VERTICES: usize = 1000;
const MAX_ENV_VERTICES: usize = 1000;

#[derive(Debug, Clone, Copy)]
pub enum BufferType {
    Paths,
    Widgets,
    Env,
}

pub struct RenderBuffers {
    pub paths: DynamicBuffer<Vertex>,
    pub widgets: DynamicBuffer<Vertex>,
    pub env: DynamicBuffer<Vertex>,
}

#[allow(dead_code)]
pub enum BufferRange {
    Full,
    OffsetFull(usize),
    Range(std::ops::Range<usize>),
}

#[derive(Debug, Clone)]
pub struct BufferLocation {
    pub offset: BufferAddress,
    pub size: BufferAddress,
    pub buffer_type: BufferType,
}

impl From<BufferLocation> for BufferRange {
    fn from(location: BufferLocation) -> Self {
        BufferRange::Range(location.offset as usize..(location.offset + location.size) as usize)
    }
}

impl RenderBuffers {
    pub fn new(device: &wgpu::Device) -> Self {
        let paths = DynamicBuffer::<Vertex>::new_init(&[], "Paths", device);
        let widgets = DynamicBuffer::<Vertex>::new(MAX_WIDGETS_VERTICES, "Widgets", device, 0..0);
        let env = DynamicBuffer::<Vertex>::new(MAX_ENV_VERTICES, "Environment", device, 0..0);

        Self {
            paths,
            widgets,
            env,
        }
    }

    pub fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        render_pass.set_vertex_buffer(0, self.paths.inner.slice(..));
        render_pass.set_vertex_buffer(1, self.widgets.inner.slice(..));
        render_pass.set_vertex_buffer(2, self.env.inner.slice(..));

        render_pass.draw(self.paths.render_range.clone(), 0..1);
        render_pass.draw(self.widgets.render_range.clone(), 0..1);
        render_pass.draw(self.env.render_range.clone(), 0..1);
    }
}

impl MeshKit for RenderBuffers {
    fn write_mesh(
        &mut self,
        queue: &wgpu::Queue,
        mesh: super::mesh::CpuMesh,
    ) -> Option<super::mesh::MeshHandle> {
        match &mesh.location().buffer_type {
            BufferType::Paths => self.paths.write_mesh(queue, mesh),
            BufferType::Widgets => self.widgets.write_mesh(queue, mesh),
            BufferType::Env => self.env.write_mesh(queue, mesh),
        }
    }

    fn init_mesh(
        &mut self,
        device: &wgpu::Device,
        mesh: super::mesh::CpuMesh,
    ) -> Option<super::mesh::MeshHandle> {
        match &mesh.location().buffer_type {
            BufferType::Paths => self.paths.init_mesh(device, mesh),
            BufferType::Widgets => self.widgets.init_mesh(device, mesh),
            BufferType::Env => self.env.init_mesh(device, mesh),
        }
    }

    fn update_mesh(
        &mut self,
        queue: &wgpu::Queue,
        handle: &super::mesh::MeshHandle,
        vertices: &[Vertex],
    ) {
        match handle.location().buffer_type {
            BufferType::Paths => self.paths.update_mesh(queue, handle, vertices),
            BufferType::Widgets => self.widgets.update_mesh(queue, handle, vertices),
            BufferType::Env => self.env.update_mesh(queue, handle, vertices),
        }
    }

    fn change_mesh(
        &mut self,
        queue: &wgpu::Queue,
        handle: &super::mesh::MeshHandle,
        change: impl Fn(&mut Vertex),
    ) {
        match handle.location().buffer_type {
            BufferType::Paths => self.paths.change_mesh(queue, handle, change),
            BufferType::Widgets => self.widgets.change_mesh(queue, handle, change),
            BufferType::Env => self.env.change_mesh(queue, handle, change),
        }
    }
}

pub struct DynamicBuffer<T> {
    inner: wgpu::Buffer,
    vertices: Vec<T>,
    render_range: std::ops::Range<u32>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> DynamicBuffer<T> {
    pub fn new(size: usize, label: &str, device: &wgpu::Device, range: std::ops::Range<u32>) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            inner,
            vertices: Vec::with_capacity(size),
            render_range: range,
        }
    }

    pub fn new_init(data: &[T], label: &str, device: &wgpu::Device) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            inner,
            vertices: data.to_vec(),
            render_range: 0..data.len() as u32,
        }
    }

    #[allow(dead_code)]
    pub fn renew(&mut self, size: usize, label: &str, device: &wgpu::Device)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.inner.destroy();

        self.inner = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.vertices = Vec::with_capacity(size);
        self.render_range = 0..size as u32;
    }

    pub fn renew_init(&mut self, data: &[T], label: &str, device: &wgpu::Device)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.inner.destroy();

        self.inner = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        self.vertices = data.to_vec();
        self.render_range = 0..data.len() as u32;
    }

    pub fn write(&mut self, queue: &wgpu::Queue, offset: BufferAddress, data: &[T])
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.vertices.splice(
            offset as usize..(offset as usize + data.len()),
            data.iter().cloned(),
        );

        // TODO optimize only write the changed vertices
        queue.write_buffer(&self.inner, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn change(&mut self, queue: &wgpu::Queue, range: BufferRange, change: impl Fn(&mut T)) {
        match range {
            BufferRange::Full => {
                for i in 0..self.vertices.len() {
                    change(&mut self.vertices[i]);
                }
            }
            BufferRange::OffsetFull(offset) => {
                for i in offset..self.vertices.len() {
                    change(&mut self.vertices[i]);
                }
            }
            BufferRange::Range(range) => {
                for i in range.clone() {
                    change(&mut self.vertices[i]);
                }
            }
        }

        // TODO optimize only write the changed vertices
        queue.write_buffer(&self.inner, 0, bytemuck::cast_slice(&self.vertices));
    }

    #[allow(dead_code)]
    pub fn read(&self, range: BufferRange) -> Option<&[T]> {
        match range {
            BufferRange::Full => Some(&self.vertices),
            BufferRange::OffsetFull(offset) => Some(&self.vertices[offset..]),
            BufferRange::Range(range) => Some(&self.vertices[range]),
        }
    }
}

impl MeshKit for DynamicBuffer<Vertex> {
    fn write_mesh(
        &mut self,
        queue: &wgpu::Queue,
        mesh: super::mesh::CpuMesh,
    ) -> Option<super::mesh::MeshHandle> {
        match &mesh {
            super::mesh::CpuMesh::Static {
                vertices, location, ..
            } => {
                self.write(queue, location.offset, vertices);
            }
            super::mesh::CpuMesh::Interactive {
                vertices, location, ..
            } => {
                self.write(queue, location.offset, vertices);
            }
        }

        Some(mesh.into())
    }

    fn init_mesh(
        &mut self,
        device: &wgpu::Device,
        mesh: super::mesh::CpuMesh,
    ) -> Option<super::mesh::MeshHandle> {
        match &mesh {
            super::mesh::CpuMesh::Static { vertices, .. } => {
                self.renew_init(vertices, "Static Mesh", device);
            }
            super::mesh::CpuMesh::Interactive { vertices, .. } => {
                self.renew_init(vertices, "Interactive Mesh", device);
            }
        }

        Some(mesh.into())
    }

    fn update_mesh(
        &mut self,
        queue: &wgpu::Queue,
        handle: &super::mesh::MeshHandle,
        vertices: &[Vertex],
    ) {
        self.write(queue, handle.location().offset, vertices);
    }

    fn change_mesh(
        &mut self,
        queue: &wgpu::Queue,
        handle: &super::mesh::MeshHandle,
        change: impl Fn(&mut Vertex),
    ) {
        let location = handle.location();

        self.change(
            queue,
            BufferRange::Range(
                location.offset as usize..(location.offset + location.size) as usize,
            ),
            change,
        )
    }
}
