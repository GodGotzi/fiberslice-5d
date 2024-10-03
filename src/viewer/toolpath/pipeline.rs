use rether::{alloc::BufferDynamicAllocator, model::geometry::Geometry, Buffer, SimpleGeometry};
use wgpu::util::DeviceExt;

use crate::{prelude::WgpuContext, slicer::print_type::PrintType};

use super::vertex::{ToolpathContext, ToolpathVertex};

#[derive(Debug)]
pub struct ToolpathBuffer {
    buffer: Buffer<ToolpathVertex, BufferDynamicAllocator<ToolpathVertex>>,
    pipeline: wgpu::RenderPipeline,

    toolpath_context_buffer: wgpu::Buffer,
    toolpath_context: ToolpathContext,
    toolpath_context_bind_group: wgpu::BindGroup,
}

impl ToolpathBuffer {
    pub fn new(
        context: &WgpuContext,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let buffer = Buffer::new("Toolpath Buffer", &context.device);

        let toolpath_context = ToolpathContext::default();

        let toolpath_context_buffer =
            context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Light VB"),
                    contents: bytemuck::cast_slice(&[toolpath_context]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let toolpath_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let toolpath_context_bind_group =
            context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &toolpath_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: toolpath_context_buffer.as_entire_binding(),
                    }],
                    label: None,
                });

        let shader = context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Toolpath Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let render_pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                        &toolpath_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[ToolpathVertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: context.surface_format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent::OVER,
                        }),

                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    ..Default::default()
                },
                multiview: None,
                cache: None,
            });

        Self {
            buffer,
            pipeline,
            toolpath_context,
            toolpath_context_bind_group,
            toolpath_context_buffer,
        }
    }

    pub fn write(
        &mut self,
        geometry: &SimpleGeometry<ToolpathVertex>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        self.buffer
            .allocate_init("TOOLPATH", geometry.build_data(), device, queue);
    }

    pub fn set_visibility(&mut self, value: u32) {
        self.toolpath_context.visibility = value;
    }

    pub fn set_visibility_type(&mut self, ty: PrintType, visible: bool) {
        let index = ty as usize;

        if visible {
            self.toolpath_context.visibility |= 1 << index;
        } else {
            self.toolpath_context.visibility &= !(1 << index);
        }
    }

    pub fn set_min_layer(&mut self, min: u32) {
        self.toolpath_context.min_layer = min;
    }

    pub fn set_max_layer(&mut self, max: u32) {
        self.toolpath_context.max_layer = max;
    }
}
