use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor, PipelineLayout, RenderPipeline, ShaderModule,
};

use super::{mesh::MeshKit, vertex::Vertex, WgpuContext};

pub mod layout;

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

    triangle_back_cull: RenderPipeline,
    triangle_no_cull: RenderPipeline,
    line: RenderPipeline,
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
    pub fn new(
        wgpu_context: &WgpuContext,
        render_pipeline_layout: PipelineLayout,
        shader: ShaderModule,
    ) -> Self {
        let paths = DynamicBuffer::<Vertex>::new_init(&[], "Paths", &wgpu_context.device);
        let widgets = DynamicBuffer::<Vertex>::new_init(&[], "Widgets", &wgpu_context.device);
        let env = DynamicBuffer::<Vertex>::new_init(&[], "Env", &wgpu_context.device);

        let triangle_back_cull =
            wgpu_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu_context.surface_format,
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
                });

        let triangle_no_cull =
            wgpu_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu_context.surface_format,
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
                        cull_mode: None,
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
                });

        let line = wgpu_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu_context.surface_format,
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
                    topology: wgpu::PrimitiveTopology::LineList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: None,
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
            });

        Self {
            paths,
            widgets,
            env,

            triangle_back_cull,
            triangle_no_cull,
            line,
        }
    }

    pub fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        render_pass.set_pipeline(&self.triangle_back_cull);
        render_pass.set_vertex_buffer(0, self.paths.inner.slice(..));
        render_pass.draw(self.paths.render_range.clone(), 0..1);

        render_pass.set_pipeline(&self.triangle_no_cull);
        render_pass.set_vertex_buffer(0, self.widgets.inner.slice(..));
        render_pass.draw(self.widgets.render_range.clone(), 0..1);

        render_pass.set_pipeline(&self.line);
        render_pass.set_vertex_buffer(0, self.env.inner.slice(..));
        render_pass.draw(self.env.render_range.clone(), 0..1);
    }
}

impl MeshKit for RenderBuffers {
    fn write_mesh(
        &mut self,
        queue: &wgpu::Queue,
        mesh: super::CpuMesh,
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
            vertices: Vec::with_capacity(size),
            render_range: 0..size as u32,
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
