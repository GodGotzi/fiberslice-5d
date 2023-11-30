use three_d::*;

use crate::{api::Contains, config, ui::data::UiData};

use super::camera::HandleOrientation;

impl Contains<LogicalPoint> for Viewport {
    fn contains(&self, point: &LogicalPoint) -> bool {
        point.x > self.x as f32
            && point.x < self.x as f32 + self.width as f32
            && point.y > self.y as f32
            && point.y < self.y as f32 + self.height as f32
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

        camera.handle_orientation(super::Orientation::Default);

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

    pub fn frame(&mut self, input: &FrameInput, data: &UiData) {
        //update viewport
        {
            if input.viewport.height != 0 && input.viewport.width != 0 {
                let height = input.viewport.height
                    - ((data.get_components().taskbar.boundary().get_height()
                        + data.get_components().modebar.boundary().get_height()
                        + data.get_components().menubar.boundary().get_height())
                        * input.device_pixel_ratio) as u32;
                //let extra = (height as f32 * 0.3) as u32;

                let viewport = Viewport {
                    x: (data.get_components().toolbar.boundary().get_width()
                        * input.device_pixel_ratio) as i32,
                    y: (((data.get_components().taskbar.boundary().get_height()
                        + data.get_components().modebar.boundary().get_height())
                        * input.device_pixel_ratio) as i32),
                    width: input.viewport.width
                        - ((data.get_components().toolbar.boundary().get_width()
                            + data.get_components().settingsbar.boundary().get_width())
                            * input.device_pixel_ratio) as u32,
                    height,
                };

                self.camera.set_viewport(viewport);
            }
        }
    }
}
