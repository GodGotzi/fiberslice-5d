use three_d::*;

use crate::config;

use super::camera::HandleOrientation;

pub struct Environment {
    camera: Camera,
    camera_control: OrbitControl,
    owned_lights: Vec<Box<dyn Light>>,
}

impl Environment {
    pub fn new(context: &Context) -> Self {
        let mut camera = crate::view::camera::CameraBuilder::new()
            .viewport(Viewport::new_at_origo(
                config::default::WINDOW_S.0 as u32,
                config::default::WINDOW_S.1 as u32,
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

        let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
        let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

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
}
