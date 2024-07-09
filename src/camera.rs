use std::f32::consts::PI;

use glam::{Mat4, Vec3};
use macros::NumEnum;
use strum_macros::{EnumCount, EnumIter};
use winit::event::WindowEvent;

use crate::{
    geometry::BoundingHitbox,
    prelude::{Adapter, FrameHandle, Viewport},
    GlobalState, RootEvent,
};

pub mod camera_controller;
pub mod orbit_camera;

pub use self::orbit_camera::OrbitCamera;
// pub use self::orbit_camera::OrbitCameraBounds;

pub use camera_controller::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, NumEnum, EnumCount, EnumIter)] //maybe performance bit worse
pub enum Orientation {
    Default,
    Diagonal,
    Top,
    Left,
    Right,
    Front,
}

#[derive(Debug, Clone)]
pub enum CameraEvent {
    CameraOrientationChanged(Orientation),
    UpdatePreferredDistance(BoundingHitbox),
}

pub struct CameraResult {
    pub view: Mat4,
    pub proj: Mat4,
    pub eye: Vec3,
    pub viewport: Viewport,
}

pub struct CameraAdapter {
    camera: OrbitCamera,
    viewport: Option<Viewport>,
    view: Mat4,
    proj: Mat4,
}

impl FrameHandle<'_, RootEvent, CameraResult, (GlobalState<RootEvent>, Viewport)>
    for CameraAdapter
{
    fn handle_frame(
        &'_ mut self,
        event: &winit::event::Event<RootEvent>,
        _start_time: std::time::Instant,
        wgpu_context: &crate::prelude::WgpuContext,
        (state, viewport): (GlobalState<RootEvent>, Viewport),
    ) -> Result<CameraResult, crate::prelude::Error> {
        state.camera_controller.write_with_fn(|controller| {
            controller.process_events(
                event,
                &wgpu_context.window,
                &mut self.camera,
                state
                    .ui_state
                    .pointer_in_use
                    .load(std::sync::atomic::Ordering::Relaxed),
            );
        });

        match event {
            winit::event::Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if viewport != self.viewport.unwrap_or_default() {
                    self.viewport = Some(viewport);
                    self.camera.aspect = viewport.2 / viewport.3;
                }

                let (view, proj) = self.camera.build_view_proj_matrix();
                self.view = view;
                self.proj = proj;
            }
            winit::event::Event::UserEvent(RootEvent::CameraEvent(event)) => match event {
                CameraEvent::CameraOrientationChanged(orientation) => {
                    self.camera.handle_orientation(*orientation);
                }
                CameraEvent::UpdatePreferredDistance(distance) => {
                    self.camera.set_preferred_distance(distance);
                }
            },
            _ => {}
        }

        Ok(CameraResult {
            view: self.view,
            proj: self.proj,
            eye: self.camera.eye,
            viewport,
        })
    }
}

impl Adapter<'_, RootEvent, (), CameraResult, (GlobalState<RootEvent>, Viewport), CameraEvent>
    for CameraAdapter
{
    fn from_context(wgpu_context: &crate::prelude::WgpuContext) -> ((), Self) {
        let mut camera = OrbitCamera::new(
            2.0,
            1.5,
            1.25,
            Vec3::new(0.0, 0.0, 0.0),
            wgpu_context.window.inner_size().width as f32
                / wgpu_context.window.inner_size().height as f32,
        );
        camera.bounds.min_distance = Some(1.1);
        camera.bounds.min_pitch = -std::f32::consts::FRAC_PI_2 + 0.1;
        camera.bounds.max_pitch = std::f32::consts::FRAC_PI_2 - 0.1;
        camera.handle_orientation(Orientation::Default);

        (
            (),
            Self {
                camera,
                viewport: None,
                view: Mat4::IDENTITY,
                proj: Mat4::IDENTITY,
            },
        )
    }

    fn get_adapter_description(&self) -> String {
        "CameraAdapter".to_string()
    }
}

/// A camera is used for rendering specific parts of the scene.
pub trait Camera: Sized {
    fn build_view_proj_matrix(&self) -> (Mat4, Mat4);
}

/// The camera uniform contains the data linked to the camera that is passed to the shader.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// The eye position of the camera in homogenous coordinates.
    ///
    /// Homogenous coordinates are used to fullfill the 16 byte alignment requirement.
    pub view_position: [f32; 4],

    /// Contains the view projection matrix.
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    /// Updates the view projection matrix of this [CameraUniform].
    ///
    /// Arguments:
    /// * `camera`: The [OrbitCamera] from which the matrix will be computed.
    pub fn update_view_proj(&mut self, view_proj: Mat4, eye: Vec3) {
        self.view_position = [eye.x, eye.y, eye.z, 1.0];
        self.view_proj = view_proj.to_cols_array_2d();
    }
}

impl Default for CameraUniform {
    /// Creates a default [CameraUniform].
    fn default() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

pub trait HandleOrientation {
    fn handle_orientation(&mut self, orientation: Orientation);
}

impl HandleOrientation for OrbitCamera {
    fn handle_orientation(&mut self, orientation: Orientation) {
        let (yaw, pitch) = match orientation {
            Orientation::Default => (PI / 8.0, PI / 4.0),
            Orientation::Diagonal => (PI / 8.0, PI / 4.0),
            Orientation::Top => (0.0, PI / 2.0),
            Orientation::Left => (PI / 2.0, 0.0),
            Orientation::Right => (-PI / 2.0, 0.0),
            Orientation::Front => (0.0, 0.0),
        };

        self.set_yaw(yaw);
        self.set_pitch(pitch);
    }
}
