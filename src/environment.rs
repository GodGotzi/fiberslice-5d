use std::{cell::RefCell, rc::Rc};

use three_d::*;

use crate::{
    api::Contains,
    config,
    event::EventReader,
    prelude::*,
    ui::{UiResult, UiState},
    view::{HandleOrientation, Orientation},
};

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

impl FrameHandle<(), (Rc<RefCell<UiState>>, UiResult)> for EnvironmentAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (state, result): (Rc<RefCell<UiState>>, UiResult),
    ) -> Result<(), Error> {
        if !result.pointer_use.unwrap_or(false) {
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

        let components = &state.borrow().components;

        if frame_input.viewport.height != 0 && frame_input.viewport.width != 0 {
            let height = frame_input.viewport.height
                - ((components.taskbar.boundary().get_height()
                    + components.modebar.boundary().get_height()
                    + components.menubar.boundary().get_height())
                    * frame_input.device_pixel_ratio) as u32;
            //let extra = (height as f32 * 0.3) as u32;

            let viewport = Viewport {
                x: (components.toolbar.boundary().get_width() * frame_input.device_pixel_ratio)
                    as i32,
                y: (((components.taskbar.boundary().get_height()
                    + components.modebar.boundary().get_height())
                    * frame_input.device_pixel_ratio) as i32),
                width: frame_input.viewport.width
                    - ((components.toolbar.boundary().get_width()
                        + components.settingsbar.boundary().get_width())
                        * frame_input.device_pixel_ratio) as u32,
                height,
            };

            self.shared_environment
                .write()
                .camera
                .set_viewport(viewport);
        }

        Ok(())
    }
}

impl Adapter<(), (Rc<RefCell<UiState>>, UiResult), EnvironmentEvent> for EnvironmentAdapter {
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

impl Environment {
    pub fn new(context: &Context) -> Self {
        let mut camera = crate::view::CameraBuilder::new()
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
