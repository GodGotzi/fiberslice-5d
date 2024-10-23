use glam::{vec3, Mat4, Vec4};
use shared::{object::ObjectMesh, SliceInput};
use slicer::Settings;
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};
use wgpu::Color;

use crate::{GlobalState, RootEvent};

#[derive(Debug, Default)]
pub struct Slicer {
    pub settings: Settings,
}

impl Slicer {
    pub fn slice(&self, global_state: &GlobalState<RootEvent>) -> Result<(), String> {
        let model_server_read = global_state.viewer.model_server.read();

        let settings = self.settings.clone();

        let models: Vec<ObjectMesh> = model_server_read.models(&settings);

        let result = slicer::slice(
            SliceInput {
                objects: models,
                fiber_intersection_objects: vec![],
            },
            &settings,
        )
        .expect("Failed to slice model");

        global_state
            .viewer
            .toolpath_server
            .write()
            .load_from_slice_result(result, settings);

        // println!("Sliced model {:?}", result);

        Ok(())
    }
}
