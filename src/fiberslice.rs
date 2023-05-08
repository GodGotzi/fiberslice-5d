pub mod utils;
pub mod window;

pub mod wgpu {
    use wgpu::{Adapter, Device, Instance, Queue, RequestDeviceError, Surface};

    pub(crate) fn request_wgpu_adapter(instance: &Instance, surface: &Surface) -> Option<Adapter> {
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
    }

    pub(crate) fn request_wgpu_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
        pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
    }

}