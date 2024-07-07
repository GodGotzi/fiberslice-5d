use alloc::BufferAllocation;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor, Device,
};

pub mod alloc;
pub mod layout;

#[allow(dead_code)]
pub enum BufferRange {
    Full,
    OffsetFull(usize),
    Range(std::ops::Range<usize>),
}

#[derive(Debug, Clone)]
pub struct BufferLocation {
    pub offset: usize,
    pub size: usize,
}

impl From<BufferLocation> for BufferRange {
    fn from(location: BufferLocation) -> Self {
        BufferRange::Range(location.offset..(location.offset + location.size))
    }
}

pub struct RawDynamicBuffer<T> {
    pub(super) inner: wgpu::Buffer,
    pub(super) render_range: std::ops::Range<u32>,

    vertices: Vec<T>,
    label: String,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> RawDynamicBuffer<T> {
    #[allow(dead_code)]
    pub fn new(size: usize, label: &str, device: &wgpu::Device) -> Self
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
            vertices: vec![T::zeroed(); size],
            render_range: 0..size as u32,
            label: label.to_string(),
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
            label: label.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn renew(&mut self, size: usize, device: &wgpu::Device)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.inner.destroy();

        self.inner = device.create_buffer(&BufferDescriptor {
            label: Some(&self.label),
            size: (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.vertices = Vec::with_capacity(size);
        self.render_range = 0..size as u32;
    }

    pub fn renew_init(&mut self, data: &[T], device: &wgpu::Device)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.inner.destroy();

        self.inner = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&self.label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        self.vertices = data.to_vec();
        self.render_range = 0..data.len() as u32;
    }

    pub fn write(&mut self, queue: &wgpu::Queue, offset: usize, data: &[T])
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        println!(
            "Writing to buffer: offset: {}, len: {}, buffer: {}. buffer len: {}",
            offset,
            data.len(),
            self.vertices.len(),
            self.label
        );
        self.vertices
            .splice(offset..(offset + data.len()), data.iter().cloned());

        let offset_bytes = offset * std::mem::size_of::<T>();

        queue.write_buffer(
            &self.inner,
            offset_bytes as u64,
            bytemuck::cast_slice(&self.vertices[offset..(offset + data.len())]),
        );
    }

    pub fn change(&mut self, queue: &wgpu::Queue, range: BufferRange, change: impl Fn(&mut T)) {
        match range {
            BufferRange::Full => {
                for i in 0..self.vertices.len() {
                    change(&mut self.vertices[i]);
                }

                queue.write_buffer(&self.inner, 0, bytemuck::cast_slice(&self.vertices));
            }
            BufferRange::OffsetFull(offset) => {
                for i in offset..self.vertices.len() {
                    change(&mut self.vertices[i]);
                }

                queue.write_buffer(
                    &self.inner,
                    offset as u64,
                    bytemuck::cast_slice(&self.vertices[offset..]),
                );
            }
            BufferRange::Range(range) => {
                for i in range.clone() {
                    change(&mut self.vertices[i]);
                }

                let offset_bytes = range.start * std::mem::size_of::<T>();

                queue.write_buffer(
                    &self.inner,
                    offset_bytes as u64,
                    bytemuck::cast_slice(&self.vertices[range]),
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn read(&self, range: BufferRange) -> &[T] {
        match range {
            BufferRange::Full => &self.vertices,
            BufferRange::OffsetFull(offset) => &self.vertices[offset..],
            BufferRange::Range(range) => &self.vertices[range],
        }
    }

    pub fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        render_pass.set_vertex_buffer(0, self.inner.slice(..));
        render_pass.draw(self.render_range.clone(), 0..1);
    }
}

pub struct DynamicBuffer<T, L> {
    inner: RawDynamicBuffer<T>,
    allocater: Box<L>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable, L: alloc::BufferAlloc<T>> DynamicBuffer<T, L> {
    pub fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        self.inner.render(render_pass);
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable, L: alloc::BufferAlloc<T>> DynamicBuffer<T, L> {
    pub fn new(allocater: L, label: &str, device: &wgpu::Device) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = RawDynamicBuffer::<T>::new(allocater.size(), label, device);

        Self {
            inner,
            allocater: Box::new(allocater),
        }
    }

    pub fn write(&mut self, queue: &wgpu::Queue, id: &str, data: &[T]) -> Option<&BufferAllocation>
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        if let Some(allocation) = self.allocater.get(id) {
            self.inner.write(queue, allocation.offset, data);

            Some(allocation)
        } else {
            None
        }
    }

    pub fn change(&mut self, queue: &wgpu::Queue, id: &str, change: impl Fn(&mut T))
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        if let Some(allocation) = self.allocater.get(id) {
            self.inner.change(
                queue,
                BufferRange::Range(allocation.offset..(allocation.offset + allocation.size)),
                change,
            );
        }
    }

    pub fn read(&self, id: &str) -> Option<&[T]> {
        if let Some(allocation) = self.allocater.get(id) {
            Some(self.inner.read(BufferRange::Range(
                allocation.offset..(allocation.offset + allocation.size),
            )))
        } else {
            None
        }
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable, L: alloc::BufferDynamicAlloc<T>> DynamicBuffer<T, L> {
    pub fn allocate(&mut self, device: &Device, id: &str, size: usize) -> &BufferAllocation
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.allocater.allocate(id, size);

        let mut current_data = self.inner.read(BufferRange::Full).to_vec();
        current_data.resize(self.allocater.size(), T::zeroed());

        self.inner.renew_init(&current_data, device);

        self.allocater.get(id).unwrap()
    }

    pub fn allocate_init(&mut self, device: &Device, id: &str, data: &[T]) -> &BufferAllocation
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.allocater.allocate(id, data.len());

        let mut current_data = self.inner.read(BufferRange::Full).to_vec();
        current_data.extend_from_slice(data);

        self.inner.renew_init(&current_data, device);

        self.allocater.get(id).unwrap()
    }

    pub fn free(&mut self, device: &Device, id: &str) {
        if let Some(allocation) = self.allocater.get(id) {
            let mut current_data = self.inner.read(BufferRange::Full).to_vec();
            current_data.drain(allocation.offset..(allocation.offset + allocation.size));

            self.inner.renew_init(&current_data, device);
        }
    }
}
