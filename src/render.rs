use std::time::Instant;

use camera::{CameraUniform, OrbitCamera};
use glam::Vec3;
use light::LightUniform;
use vertex::Vertex;
use wgpu::util::DeviceExt;

use crate::{
    environment::{camera_controller, HandleOrientation},
    model::gcode::PrintPart,
    prelude::*,
    ui::UiUpdateOutput,
    GlobalState, RootEvent,
};

pub mod camera;
pub mod light;
pub mod texture;
pub mod vertex;

const MSAA_SAMPLE_COUNT: u32 = 1;

#[derive(Debug, Clone)]
pub enum RenderEvent {
    CameraOrientationChanged(crate::environment::view::Orientation),
    AddGCodeToolpath(PrintPart),
}

struct RenderState {
    vertex_buffer: wgpu::Buffer,
    num_indices: u32,

    depth_texture_view: wgpu::TextureView,

    camera: OrbitCamera,

    camera_viewport: Option<(f32, f32, f32, f32)>,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    light_uniform: light::LightUniform,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
}

impl RenderState {
    fn update(&mut self, wgpu_context: &WgpuContext) {
        self.camera_uniform.update_view_proj(&self.camera);

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
    render_pipeline: wgpu::RenderPipeline,
    multisampled_framebuffer: wgpu::TextureView,

    egui_rpass: egui_wgpu_backend::RenderPass,

    render_state: RenderState,
}

impl FrameHandle<'_, RootEvent, (), (GlobalState<RootEvent>, Option<UiUpdateOutput>)>
    for RenderAdapter
{
    fn handle_frame(
        &mut self,
        event: &winit::event::Event<RootEvent>,
        _start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        (state, ui_output): (GlobalState<RootEvent>, Option<UiUpdateOutput>),
    ) -> Result<(), Error> {
        puffin::profile_function!();

        let pointer_in_use = state
            .ui_state
            .pointer_in_use
            .inner()
            .load(std::sync::atomic::Ordering::Relaxed);

        state.camera_controller.write_with_fn(|controller| {
            controller.process_events(
                event,
                &wgpu_context.window,
                &mut self.render_state.camera,
                pointer_in_use,
            );
        });

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    winit::event::WindowEvent::RedrawRequested => {
                        self.render_state.update(wgpu_context);

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
                            viewport,
                        } = ui_output.unwrap();

                        if viewport != self.render_state.camera_viewport.unwrap_or_default() {
                            self.render_state.camera_viewport = Some(viewport);
                            self.render_state.camera.aspect = viewport.2 / viewport.3;
                        }

                        let (x, y, width, height) = viewport;

                        if width > 0.0 && height > 0.0 {
                            render_pass.set_viewport(x, y, width, height, 0.0, 1.0);
                            // render_pass.set_scissor_rect(x as u32, y as u32, width as , height);

                            render_pass.set_pipeline(&self.render_pipeline);
                            render_pass.set_bind_group(
                                0,
                                &self.render_state.camera_bind_group,
                                &[],
                            );
                            render_pass.set_bind_group(1, &self.render_state.light_bind_group, &[]);
                            render_pass
                                .set_vertex_buffer(0, self.render_state.vertex_buffer.slice(..));

                            render_pass.draw(0..self.render_state.num_indices, 0..1);
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
                RenderEvent::CameraOrientationChanged(orientation) => {
                    self.render_state.camera.handle_orientation(*orientation);
                }
                RenderEvent::AddGCodeToolpath(part) => {
                    let vertices = part.vertices();

                    let vertex_buffer =
                        wgpu_context
                            .device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(&vertices),
                                usage: wgpu::BufferUsages::VERTEX,
                            });

                    self.render_state.vertex_buffer = vertex_buffer;
                    self.render_state.num_indices = vertices.len() as u32;
                    self.render_state
                        .camera
                        .set_best_distance(&part.bounding_box);

                    state
                        .proxy
                        .send_event(RootEvent::UiEvent(crate::ui::UiEvent::ShowSuccess(
                            "Gcode loaded".to_string(),
                        )))
                        .unwrap();

                    wgpu_context.window.request_redraw();
                }
            },
            _ => {}
        }

        Ok(())
    }
}

impl<'a>
    Adapter<'a, RootEvent, (), (), (GlobalState<RootEvent>, Option<UiUpdateOutput>), RenderEvent>
    for RenderAdapter
{
    fn from_context(context: &WgpuContext) -> ((), Self) {
        let depth_texture_view = texture::Texture::create_depth_texture(
            &context.device,
            &context.surface_config,
            MSAA_SAMPLE_COUNT,
            "depth_texture",
        );

        let mut camera_uniform = CameraUniform::default();

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

        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX,
            });

        let mut camera = OrbitCamera::new(
            2.0,
            1.5,
            1.25,
            Vec3::new(0.0, 0.0, 0.0),
            context.window.inner_size().width as f32 / context.window.inner_size().height as f32,
        );
        camera.bounds.min_distance = Some(1.1);
        camera.bounds.min_pitch = -std::f32::consts::FRAC_PI_2 + 0.1;
        camera.bounds.max_pitch = std::f32::consts::FRAC_PI_2 - 0.1;
        camera.handle_orientation(crate::environment::view::Orientation::Default);

        camera_uniform.update_view_proj(&camera);

        let render_state = RenderState {
            vertex_buffer,
            num_indices: 0,

            depth_texture_view,

            camera,

            camera_viewport: None,
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

        let render_pipeline =
            context
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
                            format: context.surface_config.format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
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
                        count: MSAA_SAMPLE_COUNT,
                        ..Default::default()
                    },
                    multiview: None,
                });

        let egui_rpass = egui_wgpu_backend::RenderPass::new(
            &context.device,
            context.surface_format,
            MSAA_SAMPLE_COUNT,
        );

        (
            (),
            RenderAdapter {
                render_pipeline,
                multisampled_framebuffer,

                egui_rpass,
                render_state,
            },
        )
    }

    fn get_adapter_description(&self) -> String {
        "RenderAdapter".to_string()
    }
}
