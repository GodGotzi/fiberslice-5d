use three_d::*;

use crate::{application::Application, config, utils::frame::FrameHandle};

use super::camera::HandleOrientation;

pub struct Environment {
    camera: Camera,
    camera_control: OrbitControl,
    owned_lights: Vec<Box<dyn Light>>,
}

impl Environment {
    pub fn new(context: &WindowedContext) -> Self {
        let mut camera = crate::view::camera::CameraBuilder::new()
            .viewport(Viewport::new_at_origo(
                config::default::WINDOW_S.width as u32,
                config::default::WINDOW_S.height as u32,
            ))
            .position(vec3(0.00, 0.0, 0.0))
            .target(vec3(0.0, 0.0, 0.0))
            .up(vec3(0.0, 1.0, 0.0))
            .fov(degrees(45.0))
            .near(0.001)
            .far(10000.0)
            .build()
            .expect("Failed to create camera");

        camera.handle_orientation(super::Orientation::Default);

        //let light0 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
        //let light1 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

        let ambient = AmbientLight::new(context, 0.2, Srgba::WHITE);
        let directional0 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(1.0, 1.0, 1.0));
        let directional1 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 1.0, 1.0));
        let directional2 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 1.0, 0.0));
        let directional3 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(1.0, 1.0, 0.0));

        let bottom = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, 0.0));

        Self {
            camera,
            camera_control: OrbitControl::new(vec3(0.0, 0.0, 0.0), 0.00001, 1000.0),
            owned_lights: vec![
                Box::new(ambient),
                Box::new(directional0),
                Box::new(directional1),
                Box::new(directional2),
                Box::new(directional3),
                Box::new(bottom),
            ],
        }
    }
}

impl Environment {
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
}

impl FrameHandle for Environment {
    fn frame(&mut self, input: &FrameInput, application: &Application) {
        //update viewport
        {
            if input.viewport.height != 0 && input.viewport.width != 0 {
                let viewport = Viewport {
                    x: (application.boundaries().toolbar.width() * input.device_pixel_ratio) as i32,
                    y: ((application.boundaries().taskbar.height()
                        + application.boundaries().modebar.height())
                        * input.device_pixel_ratio) as i32,
                    width: input.viewport.width
                        - ((application.boundaries().toolbar.width()
                            + application.boundaries().settingsbar.width())
                            * input.device_pixel_ratio) as u32,
                    height: input.viewport.height
                        - ((application.boundaries().taskbar.height()
                            + application.boundaries().modebar.height()
                            + application.boundaries().menubar.height())
                            * input.device_pixel_ratio) as u32,
                };

                self.camera.set_viewport(viewport);
            }
        }
    }
}
