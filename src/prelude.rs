use std::{fmt::Debug, sync::Arc};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use wgpu::InstanceDescriptor;
use winit::{event::Event, window::Window};

pub use crate::error::Error;
use crate::{
    environment::EnvironmentEvent,
    event::{EventReader, EventWriter},
    picking::PickingEvent,
    render::RenderEvent,
    settings::tree::QuickSettings,
    ui::UiEvent,
};

#[derive(Default, Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}

#[derive(Clone, Default, Debug)]
pub struct WrappedSharedMut<T: Debug> {
    inner: Arc<RwLock<Wrapper<T>>>,
}

impl<T: Debug> WrappedSharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Wrapper { inner })),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<Wrapper<T>> {
        self.inner.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<Wrapper<T>> {
        self.inner.write()
    }
}

#[derive(Default, Debug)]
pub struct WrappedShared<T: Debug> {
    inner: Arc<Wrapper<T>>,
}

impl<T: Debug> WrappedShared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(Wrapper { inner }),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner.inner
    }
}

#[derive(Default, Debug)]
pub struct SharedMut<T: Debug> {
    inner: Arc<RwLock<T>>,
}

impl<T: Debug> Clone for SharedMut<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug> SharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write()
    }
}

#[derive(Clone, Default)]
pub struct Shared<T> {
    inner: Arc<T>,
}

impl<T> Shared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner
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
            backends: wgpu::Backends::PRIMARY,
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

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
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

    pub fn aspect(&self) -> f32 {
        self.surface_config.width as f32 / self.surface_config.height as f32
    }
}

pub trait FrameHandle<E, T, C> {
    fn handle_frame(
        &mut self,
        event: &Event<E>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        context: C,
    ) -> Result<T, Error>;
}

pub trait Adapter<WinitE, T, C, E: Debug + Clone>: FrameHandle<WinitE, T, C> {
    fn from_context(wgpu_context: &WgpuContext) -> (EventWriter<E>, Self);

    fn get_reader(&self) -> &EventReader<E>;

    fn get_adapter_description(&self) -> String;

    fn handle_event(&mut self, event: E);

    fn handle_events(&mut self) {
        puffin::profile_function!();
        if self.get_reader().has_active_events() {
            let events = self.get_reader().read();

            for event in events {
                println!("=================");
                println!("Handling event");
                println!("Adapter: {:?}", self.get_adapter_description());
                println!("Event: {:?}", event);
                println!("=================");

                self.handle_event(event);
            }
        }
    }
}

#[derive(Clone)]
pub struct SharedSettings {
    pub main: QuickSettings,
}

impl Default for SharedSettings {
    fn default() -> Self {
        let main = QuickSettings::new("settings/main.yaml");

        Self { main }
    }
}

#[derive(Clone)]
pub struct SharedState {
    pub settings: SharedSettings,

    pub writer_ui_event: EventWriter<UiEvent>,
    pub writer_environment_event: EventWriter<EnvironmentEvent>,
    pub writer_render_event: EventWriter<RenderEvent>,
    pub writer_picking_event: EventWriter<PickingEvent>,
}

impl SharedState {
    pub fn new(
        writer_render_event: EventWriter<RenderEvent>,
        writer_environment_event: EventWriter<EnvironmentEvent>,
        writer_ui_event: EventWriter<UiEvent>,
        writer_picking_event: EventWriter<PickingEvent>,
    ) -> Self {
        Self {
            // frame_input: SharedMut::from_inner(None),
            settings: SharedSettings::default(),
            writer_ui_event,
            writer_environment_event,
            writer_render_event,
            writer_picking_event,
        }
    }
}

impl FrameHandle<(), (), ()> for SharedState {
    fn handle_frame(
        &mut self,
        _event: &Event<()>,
        start_time: std::time::Instant,
        _wgpu_context: &WgpuContext,
        _context: (),
    ) -> Result<(), Error> {
        puffin::profile_function!();
        Ok(())
    }
}
