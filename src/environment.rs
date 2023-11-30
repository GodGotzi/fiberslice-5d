use three_d::*;

use crate::{
    config,
    prelude::*,
    ui::state::UiState,
    view::{camera::HandleOrientation, Orientation},
};

pub struct EnvironmentAdapter {
    shared_environment: SharedMut<Environment>,
}

impl Adapter<()> for EnvironmentAdapter {
    fn from_context(context: &Context) -> Self {
        Self {
            shared_environment: SharedMut::from_inner(Environment::new(context)),
        }
    }
}

impl FrameHandle<UiResult> for EnvironmentAdapter {
    fn handle_frame(&mut self, frame_input: &FrameInput) -> Result<UiResult, Error> {
        let mut result = UiResult::empty();

        self.gui.update(
            &mut frame_input.events.clone(),
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                result.pointer_use = Some(ctx.is_using_pointer());
                self.screen.show(ctx, self.state.clone());
            },
        );

        Ok(result)
    }
}

impl RenderHandle for UiAdapter {
    fn handle(&self) {
        self.gui.render();
    }
}

pub struct Environment {
    camera: Camera,
    camera_control: OrbitControl,
    owned_lights: Vec<Box<dyn Light>>,
}

impl Environment {
    pub fn new(context: &Context) -> Self {
        let mut camera = crate::view::camera::CameraBuilder::new()
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

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn lights(&self) -> Vec<&dyn Light> {
        let lights: Vec<&dyn Light> = self.owned_lights.iter().map(Box::as_ref).collect();
        lights
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn handle_camera_events(&mut self, events: &mut [Event]) -> bool {
        self.camera_control.handle_events(&mut self.camera, events)
    }

    pub fn frame(&mut self, input: &FrameInput, data: &UiState) {
        //update viewport
        {
            if input.viewport.height != 0 && input.viewport.width != 0 {
                let height = input.viewport.height
                    - ((data.components.taskbar.boundary().get_height()
                        + data.components.modebar.boundary().get_height()
                        + data.components.menubar.boundary().get_height())
                        * input.device_pixel_ratio) as u32;
                //let extra = (height as f32 * 0.3) as u32;

                let viewport = Viewport {
                    x: (data.components.toolbar.boundary().get_width() * input.device_pixel_ratio)
                        as i32,
                    y: (((data.components.taskbar.boundary().get_height()
                        + data.components.modebar.boundary().get_height())
                        * input.device_pixel_ratio) as i32),
                    width: input.viewport.width
                        - ((data.components.toolbar.boundary().get_width()
                            + data.components.settingsbar.boundary().get_width())
                            * input.device_pixel_ratio) as u32,
                    height,
                };

                self.camera.set_viewport(viewport);
            }
        }
    }
}
