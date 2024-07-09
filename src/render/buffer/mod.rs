use alloc::BufferAllocation;
use log::info;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor, Device, Queue,
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

#[derive(Debug)]
pub struct RawDynamicBuffer {
    pub(super) inner: wgpu::Buffer,
    pub(super) render_range: std::ops::Range<u32>,

    size: BufferAddress,
    label: String,
}

impl RawDynamicBuffer {
    pub fn new<T>(size: usize, label: &str, device: &wgpu::Device) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        Self {
            inner,
            render_range: 0..size as u32,

            size: size as BufferAddress,
            label: label.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn new_init<T>(data: &[T], label: &str, device: &wgpu::Device) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        Self {
            inner,
            render_range: 0..data.len() as u32,

            size: data.len() as BufferAddress,
            label: label.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn allocate<T>(&mut self, size: usize, device: &wgpu::Device, queue: &wgpu::Queue)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let old_bytes = self.size * std::mem::size_of::<T>() as BufferAddress;

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some(&self.label),
            size: old_bytes + (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Buffer Copy Encoder"),
        });
        encoder.copy_buffer_to_buffer(&self.inner, 0, &buffer, 0, old_bytes);

        queue.submit(std::iter::once(encoder.finish()));

        self.inner.destroy();

        self.inner = buffer;

        self.size += size as BufferAddress;
        self.render_range = 0..self.size as u32;
    }

    pub fn append<T>(&mut self, data: &[T], device: &wgpu::Device, queue: &wgpu::Queue)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let old_bytes = self.size * std::mem::size_of::<T>() as BufferAddress;

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some(&self.label),
            size: old_bytes + std::mem::size_of_val(data) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Buffer Copy Encoder"),
        });
        encoder.copy_buffer_to_buffer(&self.inner, 0, &buffer, 0, old_bytes);

        queue.submit(std::iter::once(encoder.finish()));

        queue.write_buffer(&buffer, old_bytes, bytemuck::cast_slice(data));

        self.inner.destroy();

        self.inner = buffer;

        self.size += data.len() as BufferAddress;
        self.render_range = 0..self.size as u32;
    }

    pub fn free<T>(
        &mut self,
        offset: usize,
        size: usize,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let old_bytes = self.size * std::mem::size_of::<T>() as BufferAddress;

        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some(&self.label),
            size: old_bytes - (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let byte_offset = offset * std::mem::size_of::<T>();
        let byte_size_to_free = size * std::mem::size_of::<T>();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Buffer Copy Encoder"),
        });

        encoder.copy_buffer_to_buffer(&self.inner, 0, &buffer, 0, byte_offset as BufferAddress);

        encoder.copy_buffer_to_buffer(
            &self.inner,
            (byte_offset + byte_size_to_free) as BufferAddress,
            &buffer,
            byte_offset as BufferAddress,
            old_bytes - (byte_offset + byte_size_to_free) as BufferAddress,
        );

        queue.submit(std::iter::once(encoder.finish()));

        self.inner.destroy();

        self.inner = buffer;

        self.size -= size as BufferAddress;
        self.render_range = 0..self.size as u32;
    }

    pub fn write<T>(&mut self, queue: &wgpu::Queue, offset: usize, data: &[T])
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let offset_bytes = offset * std::mem::size_of::<T>();

        queue.write_buffer(&self.inner, offset_bytes as u64, bytemuck::cast_slice(data));
    }

    pub fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        render_pass.set_vertex_buffer(0, self.inner.slice(..));
        render_pass.draw(self.render_range.clone(), 0..1);
    }
}

#[derive(Debug)]
pub struct DynamicBuffer<T, L> {
    inner: RawDynamicBuffer,
    allocater: Box<L>,
    _phantom: std::marker::PhantomData<T>,
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
        let inner = RawDynamicBuffer::new::<T>(allocater.size(), label, device);

        Self {
            inner,
            allocater: Box::new(allocater),
            _phantom: std::marker::PhantomData,
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
}

impl<T: bytemuck::Pod + bytemuck::Zeroable, L: alloc::BufferDynamicAlloc<T>> DynamicBuffer<T, L> {
    #[allow(dead_code)]
    pub fn allocate(
        &mut self,
        id: &str,
        size: usize,
        device: &Device,
        queue: &Queue,
    ) -> &BufferAllocation
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.allocater.allocate(id, size);

        self.inner.allocate::<T>(size, device, queue);

        self.allocater.get(id).unwrap()
    }

    pub fn allocate_init(
        &mut self,
        id: &str,
        data: &[T],
        device: &Device,
        queue: &Queue,
    ) -> &BufferAllocation
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.allocater.allocate(id, data.len());

        self.inner.append(data, device, queue);

        self.allocater.get(id).unwrap()
    }

    pub fn free(&mut self, id: &str, device: &Device, queue: &Queue) {
        if let Some(allocation) = self.allocater.free(id) {
            self.inner
                .free::<T>(allocation.offset, allocation.size, device, queue);
        }
    }
}
