use shared::{object::ObjectMesh, SliceInput};
use slicer::Settings;
use tokio::task::JoinHandle;

use crate::{
    ui::{api::trim_text, custom_toasts::SLICING_PROGRESS},
    GlobalState, RootEvent,
};

#[derive(Debug, Default)]
pub struct Slicer {
    pub settings: Settings,
    handle: Option<JoinHandle<()>>,
}

impl Slicer {
    pub fn slice(&mut self, global_state: &GlobalState<RootEvent>) {
        if let Some(handle) = self.handle.take() {
            if !handle.is_finished() {
                return;
            }
        }

        let model_server_read = global_state.viewer.model_server.read();

        let settings = self.settings.clone();
        let models: Vec<ObjectMesh> = model_server_read.models(&settings);

        let global_state = global_state.clone();

        let handle = tokio::spawn(async move {
            let process = global_state
                .progress_tracker
                .write()
                .add(SLICING_PROGRESS, trim_text::<20, 4>("Slicing model"));

            let result = slicer::slice(
                SliceInput {
                    objects: models,
                    masks: vec![],
                },
                &settings,
                &process,
            )
            .expect("Failed to slice model");

            global_state
                .viewer
                .toolpath_server
                .write()
                .load_from_slice_result(result, process);

            global_state
                .ui_event_writer
                .send(crate::ui::UiEvent::ShowSuccess(
                    "Slicing finished".to_string(),
                ));
        });

        self.handle = Some(handle);
    }
}
