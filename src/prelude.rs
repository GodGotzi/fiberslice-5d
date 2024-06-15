use std::{fmt::Debug, sync::Arc};

use wgpu::InstanceDescriptor;
use winit::{event::Event, window::Window};

pub use crate::error::Error;

pub mod shared;
pub mod wrap;

pub use shared::*;
pub use wrap::*;

#[derive(Debug, Clone)]
pub struct GlobalContext {
    pub frame_time: f32,
    last_frame_time: std::time::Instant,
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self {
            frame_time: 0.0,
            last_frame_time: std::time::Instant::now(),
        }
    }
}

impl GlobalContext {
    pub fn begin_frame(&mut self) {
        self.last_frame_time = std::time::Instant::now();
    }

    pub fn end_frame(&mut self) {
        self.frame_time = self.last_frame_time.elapsed().as_secs_f32();

        println!("Frame time: {}", self.frame_time);
    }
}

pub struct WgpuContext<'a> {
    pub window: Arc<Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,

    pub surface: wgpu::Surface<'a>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,
}

impl WgpuContext<'_> {
    pub fn new(window: Arc<Window>) -> Result<Self, Error> {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();

        // WGPU 0.11+ support force fallback (if HW implementation not supported), set it to true or false (optional).
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        println!("Adapter: {:?}", adapter.get_info());

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits {
                    max_buffer_size: 1024 * 1024 * 1024,
                    ..Default::default()
                },
                label: None,
            },
            None,
        ))
        .unwrap();

        let size = window.inner_size();
        let surface_format = surface.get_capabilities(&adapter).formats[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            desired_maximum_frame_latency: 1,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_format],
        };
        surface.configure(&device, &surface_config);

        Ok(Self {
            window,
            device,
            queue,
            adapter,
            surface,
            surface_config,
            surface_format,
        })
    }
}

pub trait FrameHandle<'a, E, T, C> {
    fn handle_frame(
        &'a mut self,
        event: &Event<E>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        state: C,
    ) -> Result<T, Error>;
}

pub trait Adapter<'a, WinitE, S: Sized, T, C, E: Debug + Clone>:
    FrameHandle<'a, WinitE, T, C>
{
    fn from_context(wgpu_context: &WgpuContext) -> (S, Self);

    #[allow(dead_code)]
    fn get_adapter_description(&self) -> String;
}
