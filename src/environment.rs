use std::{f32::consts::PI, fmt::Debug};

pub mod camera_controller;
pub mod view;

use crate::{
    event::EventReader,
    prelude::*,
    render::camera::OrbitCamera,
    ui::{parallel::ParallelUiOutput, UiState},
};

use glam::vec3;
use view::Orientation;
use winit::event;

#[derive(Debug, Clone)]
pub enum EnvironmentEvent {
    SendOrientation(Orientation),
}

pub struct EnvironmentAdapter {
    shared_environment: SharedMut<Environment>,
    event_reader: EventReader<EnvironmentEvent>,
}

impl EnvironmentAdapter {
    pub fn share_environment(&self) -> SharedMut<Environment> {
        self.shared_environment.clone()
    }
}

impl FrameHandle<(), (), (SharedMut<UiState>, &Result<ParallelUiOutput, Error>)>
    for EnvironmentAdapter
{
    fn handle_frame(
        &mut self,
        event: &winit::event::Event<()>,
        wgpu_context: WgpuContext,
        (_state, result): (SharedMut<UiState>, &Result<ParallelUiOutput, Error>),
    ) -> Result<(), Error> {
        puffin::profile_function!();

        Ok(())
    }
}

impl Adapter<(), (), (SharedMut<UiState>, &Result<ParallelUiOutput, Error>), EnvironmentEvent>
    for EnvironmentAdapter
{
    fn from_context(context: &WgpuContext) -> (crate::event::EventWriter<EnvironmentEvent>, Self) {
        let (reader, writer) = crate::event::create_event_bundle::<EnvironmentEvent>();

        (
            writer,
            Self {
                shared_environment: SharedMut::from_inner(Environment::new(context)),
                event_reader: reader,
            },
        )
    }

    fn get_reader(&self) -> &EventReader<EnvironmentEvent> {
        &self.event_reader
    }

    fn handle_event(&mut self, event: EnvironmentEvent) {
        match event {
            EnvironmentEvent::SendOrientation(orientation) => {
                self.shared_environment
                    .write()
                    .camera
                    .handle_orientation(orientation);
            }
        }
    }

    fn get_adapter_description(&self) -> String {
        "EnvironmentAdapter".to_string()
    }
}

pub struct Environment {
    camera: OrbitCamera,
    controller: camera_controller::CameraController,
}

impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("camera", &self.camera)
            .finish()
    }
}

impl Environment {
    pub fn new(context: &WgpuContext) -> Self {
        let camera = OrbitCamera::new(100.0, 1.5, 1.25, vec3(0.0, 0.0, 0.0), context.aspect());

        // camera.handle_orientation(Orientation::Default);

        //let light0 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
        //let light1 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

        //let bottom = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, 0.0));

        let controller = camera_controller::CameraController::new(5.0, 5.0);

        Self { camera, controller }
    }
}

pub trait HandleOrientation {
    fn handle_orientation(&mut self, orientation: Orientation);
}

impl HandleOrientation for OrbitCamera {
    fn handle_orientation(&mut self, orientation: Orientation) {
        let (distance, yaw, pitch) = match orientation {
            Orientation::Default => (100.0, 1.5, 1.25),
            Orientation::Diagonal => (100.0, PI / 4.0, PI / 4.0),
            Orientation::Top => (100.0, 0.0, PI / 2.0),
            Orientation::Left => (100.0, PI / 2.0, 0.0),
            Orientation::Right => (100.0, -PI / 2.0, 0.0),
            Orientation::Front => (100.0, 0.0, 0.0),
        };

        self.set_distance(distance);
        self.set_yaw(yaw);
        self.set_pitch(pitch);
    }
}
