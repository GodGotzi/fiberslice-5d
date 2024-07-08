use std::{sync::Arc, time::Instant};

use buffer::{
    alloc::BufferDynamicAllocator,
    layout::{wire::WireAllocator, WidgetAllocator},
    BufferLocation, DynamicBuffer,
};
use glam::{Mat4, Vec3};
use light::LightUniform;
use model::Model;
use vertex::Vertex;
use wgpu::util::DeviceExt;

use crate::{
    camera::{self, CameraResult, CameraUniform},
    geometry::{
        mesh::{Mesh, WireMesh},
        SelectBox,
    },
    model::gcode::{compute_normals, PrintPart, TestContext},
    prelude::*,
    ui::UiUpdateOutput,
    GlobalState, RootEvent,
};

pub mod buffer;
pub mod light;
pub mod model;
pub mod texture;
pub mod vertex;

const MSAA_SAMPLE_COUNT: u32 = 1;

#[derive(Debug, Clone)]
pub enum RenderEvent {
    AddGCodeToolpath(PrintPart),
    LoadMesh(Model<Vertex>),
    Debug(String, Vec<Vec3>),
    DebugVertex,
}

struct RenderState {
    depth_texture_view: wgpu::TextureView,

    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    light_uniform: light::LightUniform,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
}

impl RenderState {
    fn update(&mut self, wgpu_context: &WgpuContext, view_proj: Mat4, eye: Vec3) {
        self.camera_uniform.update_view_proj(view_proj, eye);

        wgpu_context.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // Update the light so that it is transformed with the camera
        self.light_uniform.position = [
            self.camera_uniform.view_position[0],
            self.camera_uniform.view_position[1],
            self.camera_uniform.view_position[2],
            1.0,
        ];
        wgpu_context.queue.write_buffer(
            &self.light_buffer,
            0,
            bytemuck::cast_slice(&[self.light_uniform]),
        );
    }
}

pub struct RenderAdapter {
    multisampled_framebuffer: wgpu::TextureView,

    egui_rpass: egui_wgpu_backend::RenderPass,

    back_cull_pipline: wgpu::RenderPipeline,
    no_cull_pipline: wgpu::RenderPipeline,
    wire_pipline: wgpu::RenderPipeline,

    wire_buffer: DynamicBuffer<Vertex, WireAllocator>,
    widget_buffer: DynamicBuffer<Vertex, WidgetAllocator>,
    main_buffer: DynamicBuffer<Vertex, BufferDynamicAllocator>,

    debug_buffer: DynamicBuffer<Vertex, BufferDynamicAllocator>,

    render_state: RenderState,
}

impl<'a>
    FrameHandle<
        'a,
        RootEvent,
        (),
        (
            GlobalState<RootEvent>,
            Option<UiUpdateOutput>,
            &CameraResult,
        ),
    > for RenderAdapter
{
    fn handle_frame(
        &'a mut self,
        event: &winit::event::Event<RootEvent>,
        _start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        (state, ui_output, camera_result): (
            GlobalState<RootEvent>,
            Option<UiUpdateOutput>,
            &CameraResult,
        ),
    ) -> Result<(), Error> {
        puffin::profile_function!("Render handle_frame");

        let CameraResult {
            view,
            proj,
            eye,
            viewport,
        } = *camera_result;

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    winit::event::WindowEvent::RedrawRequested => {
                        self.render_state.update(wgpu_context, proj * view, eye);

                        let now = Instant::now();

                        let output = wgpu_context
                            .surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = wgpu_context.device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            },
                        );

                        let clear_color = wgpu::Color {
                            r: 0.4,
                            g: 0.5,
                            b: 0.4,
                            a: 1.0,
                        };

                        let rpass_color_attachment = wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_color),
                                store: wgpu::StoreOp::Store,
                            },
                        };

                        let mut render_pass =
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Render Pass"),
                                color_attachments: &[Some(rpass_color_attachment)],
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: &self.render_state.depth_texture_view,
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(1.0),
                                            store: wgpu::StoreOp::Store,
                                        }),
                                        stencil_ops: None,
                                    },
                                ),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                        let UiUpdateOutput {
                            paint_jobs,
                            tdelta,
                            screen_descriptor,
                        } = ui_output.unwrap();

                        let (x, y, width, height) = viewport;

                        if width > 0.0 && height > 0.0 {
                            render_pass.set_viewport(x, y, width, height, 0.0, 1.0);
                            // render_pass.set_scissor_rect(x as u32, y as u32, width as , height);
                            // render_pass.set_pipeline(&self.render_pipeline);

                            render_pass.set_bind_group(
                                0,
                                &self.render_state.camera_bind_group,
                                &[],
                            );
                            render_pass.set_bind_group(1, &self.render_state.light_bind_group, &[]);

                            render_pass.set_pipeline(&self.back_cull_pipline);
                            self.main_buffer.render(&mut render_pass);

                            render_pass.set_pipeline(&self.no_cull_pipline);
                            self.widget_buffer.render(&mut render_pass);

                            render_pass.set_pipeline(&self.wire_pipline);
                            self.wire_buffer.render(&mut render_pass);
                            self.debug_buffer.render(&mut render_pass);
                        }

                        self.egui_rpass
                            .add_textures(&wgpu_context.device, &wgpu_context.queue, &tdelta)
                            .expect("add texture ok");

                        self.egui_rpass.update_buffers(
                            &wgpu_context.device,
                            &wgpu_context.queue,
                            &paint_jobs,
                            &screen_descriptor,
                        );

                        drop(render_pass);

                        self.egui_rpass
                            .execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)
                            .expect("execute render pass ok");

                        wgpu_context.queue.submit(std::iter::once(encoder.finish()));
                        output.present();

                        self.egui_rpass
                            .remove_textures(tdelta)
                            .expect("remove texture ok");

                        println!("Render time: {:?}", now.elapsed());
                    }
                    winit::event::WindowEvent::Resized(size) => {
                        if size.width > 0 && size.height > 0 {
                            self.render_state.depth_texture_view =
                                texture::Texture::create_depth_texture(
                                    &wgpu_context.device,
                                    &wgpu_context.surface_config,
                                    MSAA_SAMPLE_COUNT,
                                    "depth_texture",
                                );
                            self.multisampled_framebuffer =
                                texture::Texture::create_multisampled_framebuffer(
                                    &wgpu_context.device,
                                    &wgpu_context.surface_config,
                                    MSAA_SAMPLE_COUNT,
                                    "multisampled_framebuffer",
                                );
                        }
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                        let size = wgpu_context.window.inner_size();

                        if size.width > 0 && size.height > 0 {
                            self.render_state.depth_texture_view =
                                texture::Texture::create_depth_texture(
                                    &wgpu_context.device,
                                    &wgpu_context.surface_config,
                                    MSAA_SAMPLE_COUNT,
                                    "depth_texture",
                                );
                            self.multisampled_framebuffer =
                                texture::Texture::create_multisampled_framebuffer(
                                    &wgpu_context.device,
                                    &wgpu_context.surface_config,
                                    MSAA_SAMPLE_COUNT,
                                    "multisampled_framebuffer",
                                );
                        }
                    }
                    _ => {}
                }
            }
            winit::event::Event::UserEvent(RootEvent::RenderEvent(event)) => match event {
                RenderEvent::AddGCodeToolpath(part) => {
                    let vertices = part.vertices();

                    let box_vertices = SelectBox::from(part.bounding_box).to_triangle_vertices();

                    let mut vertex_box = box_vertices
                        .iter()
                        .map(|vertex| Vertex {
                            position: vertex.to_array(),
                            normal: [0.0, 0.0, 0.0],
                            color: [0.0, 0.0, 1.0, 1.0],
                        })
                        .collect::<Vec<Vertex>>();

                    compute_normals(&box_vertices, &mut vertex_box);

                    let line_vertices = SelectBox::from(part.bounding_box)
                        .to_wire_vertices()
                        .iter()
                        .map(|vertex| Vertex {
                            position: vertex.to_array(),
                            normal: [0.0, 0.0, 0.0],
                            color: [0.0, 0.0, 1.0, 1.0],
                        })
                        .collect::<Vec<Vertex>>();

                    self.main_buffer
                        .free("entity-1", &wgpu_context.device, &wgpu_context.queue);

                    self.main_buffer.allocate_init(
                        "entity-1",
                        &vertices,
                        &wgpu_context.device,
                        &wgpu_context.queue,
                    );

                    self.wire_buffer
                        .write(&wgpu_context.queue, "select_box", &line_vertices);

                    self.widget_buffer
                        .write(&wgpu_context.queue, "select_box", &vertex_box);

                    state
                        .proxy
                        .send_event(RootEvent::CameraEvent(
                            camera::CameraEvent::UpdatePreferredDistance(part.bounding_box),
                        ))
                        .unwrap();

                    state
                        .proxy
                        .send_event(RootEvent::PickingEvent(
                            crate::picking::PickingEvent::AddInteractiveMesh(
                                model::MeshHandle::Interactive {
                                    location: BufferLocation { offset: 0, size: 1 },
                                    sub_meshes: Vec::new(),
                                    raw_box: part.bounding_box,
                                    context: Arc::new(Box::new(TestContext {})),
                                },
                            ),
                        ))
                        .unwrap();

                    state
                        .proxy
                        .send_event(RootEvent::UiEvent(crate::ui::UiEvent::ShowSuccess(
                            "Gcode loaded".to_string(),
                        )))
                        .unwrap();

                    wgpu_context.window.request_redraw();
                }
                RenderEvent::LoadMesh(mesh) => {
                    let vertices = match mesh {
                        Model::Static { vertices, .. } => vertices,
                        Model::Interactive { vertices, .. } => vertices,
                    };

                    self.wire_buffer
                        .write(&wgpu_context.queue, "ray_debug", vertices);

                    wgpu_context.window.request_redraw();
                }
                RenderEvent::Debug(name, vertices) => {
                    let data = vertices
                        .iter()
                        .map(|v| Vertex {
                            position: v.to_array(),
                            normal: [0.0, 0.0, 0.0],
                            color: [1.0, 0.0, 0.0, 1.0],
                        })
                        .collect::<Vec<Vertex>>();

                    // compute_normals(vertices, &mut data);

                    self.debug_buffer.allocate_init(
                        name,
                        &data,
                        &wgpu_context.device,
                        &wgpu_context.queue,
                    );

                    wgpu_context.window.request_redraw();
                }
                RenderEvent::DebugVertex => {
                    wgpu_context.window.request_redraw();
                }
            },
            _ => {}
        }

        Ok(())
    }
}

impl<'a>
    Adapter<
        'a,
        RootEvent,
        (),
        (),
        (
            GlobalState<RootEvent>,
            Option<UiUpdateOutput>,
            &CameraResult,
        ),
        RenderEvent,
    > for RenderAdapter
{
    fn from_context(context: &WgpuContext) -> ((), Self) {
        let depth_texture_view = texture::Texture::create_depth_texture(
            &context.device,
            &context.surface_config,
            MSAA_SAMPLE_COUNT,
            "depth_texture",
        );

        let camera_uniform = CameraUniform::default();

        let camera_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let camera_bind_group_layout =
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
                    label: Some("camera_bind_group_layout"),
                });

        let camera_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        let light_uniform = LightUniform {
            position: [1000.0, 1000.0, 1000.0, 1.0],
            color: [1.0, 1.0, 1.0, 0.1],
        };

        let light_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let light_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });

        let light_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &light_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_buffer.as_entire_binding(),
                }],
                label: None,
            });

        let render_state = RenderState {
            depth_texture_view,

            camera_uniform,
            camera_buffer,
            camera_bind_group,

            light_uniform,
            light_buffer,
            light_bind_group,
        };

        let shader = context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("render/shader.wgsl").into()),
            });

        let render_pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let multisampled_framebuffer = texture::Texture::create_multisampled_framebuffer(
            &context.device,
            &context.surface_config,
            MSAA_SAMPLE_COUNT,
            "multisampled_framebuffer",
        );

        let egui_rpass = egui_wgpu_backend::RenderPass::new(
            &context.device,
            context.surface_format,
            MSAA_SAMPLE_COUNT,
        );

        let widget_buffer = DynamicBuffer::new(WidgetAllocator, "Widget Buffer", &context.device);

        let wire_buffer = DynamicBuffer::new(WireAllocator, "Wire Buffer", &context.device);

        let main_buffer = DynamicBuffer::new(
            BufferDynamicAllocator::default(),
            "Main Buffer",
            &context.device,
        );

        let debug_buffer = DynamicBuffer::new(
            BufferDynamicAllocator::default(),
            "Debug Buffer",
            &context.device,
        );

        (
            (),
            RenderAdapter {
                multisampled_framebuffer,

                egui_rpass,

                back_cull_pipline: create_pipline(
                    context,
                    &render_pipeline_layout,
                    &shader,
                    wgpu::PrimitiveTopology::TriangleList,
                    Some(wgpu::Face::Back),
                ),
                no_cull_pipline: create_pipline(
                    context,
                    &render_pipeline_layout,
                    &shader,
                    wgpu::PrimitiveTopology::TriangleList,
                    None,
                ),
                wire_pipline: create_pipline(
                    context,
                    &render_pipeline_layout,
                    &shader,
                    wgpu::PrimitiveTopology::LineList,
                    None,
                ),

                wire_buffer,
                widget_buffer,
                main_buffer,
                debug_buffer,

                render_state,
            },
        )
    }

    fn get_adapter_description(&self) -> String {
        "RenderAdapter".to_string()
    }
}

fn create_pipline(
    context: &WgpuContext,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
) -> wgpu::RenderPipeline {
    context
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
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
                topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode,
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
        })
}
