use std::fmt::Debug;

use three_d::*;

pub mod view;

use crate::{
    api::Contains,
    config,
    event::EventReader,
    prelude::*,
    ui::{parallel::ParallelUiOutput, UiState},
};

use view::Orientation;

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

impl FrameHandle<(), (SharedMut<UiState>, &Result<ParallelUiOutput, Error>)>
    for EnvironmentAdapter
{
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (_state, result): (SharedMut<UiState>, &Result<ParallelUiOutput, Error>),
    ) -> Result<(), Error> {
        if let Ok(result) = result {
            if !result.pointer_use {
                let mut events = frame_input
                    .events
                    .clone()
                    .into_iter()
                    .filter(|event| {
                        let position = match event {
                            Event::MousePress { position, .. } => position,
                            Event::MouseRelease { position, .. } => position,
                            Event::MouseMotion { position, .. } => position,
                            Event::MouseWheel { position, .. } => position,
                            _ => return true,
                        };

                        self.shared_environment
                            .read()
                            .camera
                            .viewport()
                            .contains(position)
                    })
                    .collect::<Vec<Event>>();

                self.shared_environment
                    .write()
                    .handle_camera_events(&mut events);
            }

            if result.camera_viewport.height > 0 && result.camera_viewport.width > 0 {
                self.shared_environment
                    .write()
                    .camera
                    .set_viewport(result.camera_viewport);
            }
        }

        Ok(())
    }
}

impl Adapter<(), (SharedMut<UiState>, &Result<ParallelUiOutput, Error>), EnvironmentEvent>
    for EnvironmentAdapter
{
    fn from_context(context: &Context) -> (crate::event::EventWriter<EnvironmentEvent>, Self) {
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
    camera: Camera,
    camera_control: OrbitControl,
    owned_lights: Vec<Box<dyn Light>>,
}

impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("camera", &self.camera)
            .finish()
    }
}

impl Environment {
    pub fn new(context: &Context) -> Self {
        let mut camera = view::CameraBuilder::new()
            .viewport(Viewport::new_at_origo(
                config::default::WINDOW_S.0,
                config::default::WINDOW_S.1,
            ))
            .position(vec3(0.00, 0.0, 0.0))
            .target(vec3(0.0, 0.0, 0.0))
            .up(vec3(0.0, 1.0, 0.0))
            .fov(degrees(45.0))
            .near(1.0)
            .far(10000.0)
            .build()
            .expect("Failed to create camera");

        camera.handle_orientation(Orientation::Default);

        //let light0 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
        //let light1 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

        let light0 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
        let light1 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

        //let bottom = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, 0.0));

        Self {
            camera,
            camera_control: OrbitControl::new(vec3(0.0, 0.0, 0.0), 0.00001, 1000.0),
            owned_lights: vec![Box::new(light0), Box::new(light1)],
        }
    }

    pub fn handle_camera_events(&mut self, events: &mut [Event]) -> bool {
        self.camera_control.handle_events(&mut self.camera, events)
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn lights(&self) -> Vec<&dyn Light> {
        let lights: Vec<&dyn Light> = self.owned_lights.iter().map(Box::as_ref).collect();
        lights
    }
}

pub trait HandleOrientation {
    fn handle_orientation(&mut self, orientation: Orientation);
}

impl HandleOrientation for Camera {
    fn handle_orientation(&mut self, orientation: Orientation) {
        let position = match orientation {
            Orientation::Default => vec3(0.00, 250.0, 500.0),
            Orientation::Diagonal => vec3(500.0, 500.0, 500.0),
            Orientation::Top => vec3(0.0, 900.0, 0.1), // FIXME 0.1 is a hack to avoid the camera being inside the model, maybe there is a better way to do this
            Orientation::Left => vec3(-500.0, 0.0, 0.0),
            Orientation::Right => vec3(500.0, 0.0, 0.0),
            Orientation::Front => vec3(0.0, 0.0, 500.0),
        };

        self.set_view(position, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0))
    }
}
