use glam::{Mat4, Quat, Vec3};
use rether::{Rotate, Scale, Transform, Translate};
use wgpu::util::DeviceExt;

use crate::{DEVICE, QUEUE};

pub const TRANSFORM_INDEX: u32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub transform: [[f32; 4]; 4],
}

#[derive(Debug)]
pub enum ModelState {
    Dormant,
    Awake(wgpu::Buffer, u32),
}

#[derive(Debug)]
pub struct Model<T> {
    state: ModelState,
    transform: Mat4,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: std::fmt::Debug + bytemuck::Pod + bytemuck::Zeroable> Model<T> {
    pub fn create() -> Self {
        let device_read = DEVICE.read();
        let device = device_read.as_ref().unwrap();

        let transform = Mat4::from_translation(Vec3::ZERO);

        let transform_uniform = TransformUniform {
            transform: transform.to_cols_array_2d(),
        };

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            state: ModelState::Dormant,
            transform,
            transform_buffer,
            transform_bind_group,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get_transform(&self) -> Mat4 {
        self.transform
    }

    pub fn awaken(&mut self, data: &[T]) {
        let device_read = DEVICE.read();
        let device = device_read.as_ref().unwrap();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.state = ModelState::Awake(buffer, data.len() as u32);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        let (buffer, size) = match &self.state {
            ModelState::Dormant => return,
            ModelState::Awake(buffer, size) => (buffer, size),
        };

        render_pass.set_bind_group(TRANSFORM_INDEX, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, buffer.slice(..));
        render_pass.draw(0..*size, 0..1);
    }
}

impl<T> Drop for Model<T> {
    fn drop(&mut self) {
        match &self.state {
            ModelState::Dormant => {}
            ModelState::Awake(buffer, ..) => {
                buffer.destroy();
            }
        }
    }
}

impl<T> Translate for Model<T> {
    fn translate(&mut self, translation: Vec3) {
        let queue_read = QUEUE.read();
        let queue = queue_read.as_ref().unwrap();

        self.transform *= Mat4::from_translation(translation);
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}

impl<T> Rotate for Model<T> {
    fn rotate(&mut self, rotation: Quat) {
        let queue_read = QUEUE.read();
        let queue = queue_read.as_ref().unwrap();

        self.transform *= Mat4::from_quat(rotation);
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}

impl<T> Scale for Model<T> {
    fn scale(&mut self, scale: Vec3) {
        let queue_read = QUEUE.read();
        let queue = queue_read.as_ref().unwrap();

        self.transform *= Mat4::from_scale(scale);
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}

impl<T> Transform for Model<T> {
    fn transform(&mut self, transform: Mat4) {
        let queue_read = QUEUE.read();
        let queue = queue_read.as_ref().unwrap();

        self.transform = transform;
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}
