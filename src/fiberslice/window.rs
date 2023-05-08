use wgpu::{Device, Surface, SurfaceConfiguration};
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::Window;

#[allow(dead_code)]
pub struct WindowEventObserver<'a> {
    window: &'a Window,
}

pub struct FiberSliceWindow<'a> {
    window: &'a Window,
    device: &'a Device,
    surface: &'a Surface,
    surface_config: &'a mut SurfaceConfiguration,
}

impl <'a> WindowEventObserver<'a> {

    pub fn new(window: &'a Window) -> Self {
        WindowEventObserver {
            window
        }
    }

    pub fn call_event(&self, event: WindowEvent, fiber_slice_window: &mut FiberSliceWindow, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(size) => {
                // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                // See: https://github.com/rust-windowing/winit/issues/208
                // This solves an issue where the app would panic when minimizing on Windows.
                if size.width > 0 && size.height > 0 {
                    fiber_slice_window.surface_config.width = size.width;
                    fiber_slice_window.surface_config.height = size.height;
                    fiber_slice_window.surface.configure(fiber_slice_window.device, fiber_slice_window.surface_config);
                }
            }
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {

            }
        }
    }
}

#[allow(dead_code)]
impl <'a> FiberSliceWindow<'a> {

    pub fn new(window: &'a Window, device: &'a Device, surface: &'a Surface, surface_config: &'a mut SurfaceConfiguration) -> Self {
        FiberSliceWindow {
            window,
            device,
            surface,
            surface_config,
        }
    }

    pub fn get_window(&self) -> &Window {
        self.window
    }

    pub fn get_surface(&self) -> &Surface {
        self.surface
    }

    pub fn get_surface_config(&self) -> &SurfaceConfiguration {
        self.surface_config
    }

    pub fn get_device(&self) -> &Device {
        self.device
    }
}