use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor,
};

use super::vertex::Vertex;

const MAX_WIDGETS_VERTICES: usize = 1000;
const MAX_ENV_VERTICES: usize = 1000;

pub struct RenderBuffers {
    pub paths: DynamicBuffer,
    pub widgets: DynamicBuffer,
    pub env: DynamicBuffer,
}

impl RenderBuffers {
    pub fn new(device: &wgpu::Device) -> Self {
        let paths = DynamicBuffer::new_init::<Vertex>(&[], "Paths", device);
        let widgets = DynamicBuffer::new::<Vertex>(MAX_WIDGETS_VERTICES, "Widgets", device, 0..0);
        let env = DynamicBuffer::new::<Vertex>(MAX_ENV_VERTICES, "Environment", device, 0..0);

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

pub struct DynamicBuffer {
    inner: wgpu::Buffer,
    render_range: std::ops::Range<u32>,
}

impl DynamicBuffer {
    pub fn new<T>(
        size: usize,
        label: &str,
        device: &wgpu::Device,
        range: std::ops::Range<u32>,
    ) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        Self {
            inner,
            render_range: range,
        }
    }

    pub fn new_init<T>(data: &[T], label: &str, device: &wgpu::Device) -> Self
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let inner = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            inner,
            render_range: 0..data.len() as u32,
        }
    }

    pub fn renew<T>(&mut self, size: usize, label: &str, device: &wgpu::Device)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.inner.destroy();

        self.inner = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: (size * std::mem::size_of::<T>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        self.render_range = 0..size as u32;
    }

    pub fn renew_init<T>(&mut self, data: &[T], label: &str, device: &wgpu::Device)
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        self.inner.destroy();

        self.inner = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.render_range = 0..data.len() as u32;
    }

    pub fn write<T>(&self, queue: &wgpu::Queue, offset: BufferAddress, data: &[T])
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        queue.write_buffer(&self.inner, offset, bytemuck::cast_slice(data));
    }

    pub fn change<SS, T>(
        &self,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        range: SS,
        change: impl Fn(&mut T),
    ) where
        SS: std::ops::RangeBounds<BufferAddress>,
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        pollster::block_on(async {
            let offset = match range.start_bound() {
                std::ops::Bound::Included(offset) => *offset,
                std::ops::Bound::Excluded(offset) => offset + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let mut data = self.read(device, range).await;

            data.iter_mut().for_each(change);

            queue.write_buffer(&self.inner, offset, bytemuck::cast_slice(&data));
        });
    }

    pub async fn read<SS, T>(&self, device: &wgpu::Device, range: SS) -> Vec<T>
    where
        SS: std::ops::RangeBounds<BufferAddress>,
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let (sender, receiver) = futures_channel::oneshot::channel();

        let slice = self.inner.slice(range);
        slice.map_async(wgpu::MapMode::Read, |res| {
            let _ = sender.send(res);
        });
        device.poll(wgpu::Maintain::Wait);

        receiver
            .await
            .expect("Failed to read buffer")
            .expect("Failed to read buffer");

        bytemuck::cast_slice(&slice.get_mapped_range()).to_vec()
    }

    pub fn update_range(&mut self, range: std::ops::Range<u32>) {
        self.render_range = range;
    }
}
