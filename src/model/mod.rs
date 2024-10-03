use std::sync::Arc;

use glam::{Mat4, Quat, Vec3};
use rether::{Rotate, Translate};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Queue,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub transform: [[f32; 4]; 4],
}

pub struct Model {
    buffer: wgpu::Buffer,
    queue: Arc<Queue>,
    transform: Mat4,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
}

impl Model {
    pub fn new(device: &wgpu::Device, queue: Arc<Queue>) -> Self {
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

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            buffer,
            queue,
            transform,
            transform_buffer,
            transform_bind_group,
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        self.buffer.destroy();
    }
}

impl Translate for Model {
    fn translate(&mut self, translation: Vec3) {
        self.transform = self.transform * Mat4::from_translation(translation);
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}

impl Rotate for Model {
    fn rotate(&mut self, rotation: Quat) {
        self.transform = self.transform * Mat4::from_quat(rotation);
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}

impl Scale for Model {
    fn scale(&mut self, scale: Vec3) {
        self.transform = self.transform * Mat4::from_scale(scale);
        let transform_uniform = TransformUniform {
            transform: self.transform.to_cols_array_2d(),
        };

        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_uniform]),
        );
    }
}
