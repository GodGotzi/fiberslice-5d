use three_d::*;

use crate::config;

pub struct Environment {
    camera: Camera,
    owned_lights: Vec<Box<dyn Light>>,
}

impl Environment {
    pub fn new(context: &WindowedContext) -> Self {
        let mut camera = crate::view::camera::CameraBuilder::new()
            .viewport(Viewport::new_at_origo(
                config::default::WINDOW_S.width as u32,
                config::default::WINDOW_S.height as u32,
            ))
            .position(vec3(60.00, 50.0, 60.0))
            .target(vec3(0.0, 0.0, 0.0))
            .up(vec3(0.0, 1.0, 0.0))
            .fov(degrees(45.0))
            .near(0.001)
            .far(10000.0)
            .build()
            .expect("Failed to create camera");

        camera.zoom_towards(&vec3(0.0, 0.0, 0.0), -400.0, 0.00001, 1000.0);

        let light0 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
        let light1 = DirectionalLight::new(context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

        Self {
            camera,
            owned_lights: vec![Box::new(light0), Box::new(light1)],
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
}
