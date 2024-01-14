use three_d::{ClearState, Context, FrameInput, RenderTarget};

use crate::{
    environment::Environment,
    event::{create_event_bundle, EventReader, EventWriter},
    model::gcode::PrintPart,
    prelude::*,
    ui::{UiAdapter, UiResult},
};

#[derive(Debug, Clone)]
pub enum RenderEvent {}

pub struct RenderState {
    workpiece: SharedMut<Option<PrintPart>>,
}

impl Clone for RenderState {
    fn clone(&self) -> Self {
        Self {
            workpiece: self.workpiece.clone(),
        }
    }
}

pub struct RenderAdapter {
    shared_state: RenderState,
    event_reader: EventReader<RenderEvent>,
}

impl RenderAdapter {
    pub fn share_state(&self) -> RenderState {
        self.shared_state.clone()
    }

    pub fn render(&mut self, environment: &Environment) {

        /*
               state
           .toolpath
           .as_mut()
           .unwrap()
           .render(environment.camera(), environment.lights().as_slice());
        */
    }
}

impl FrameHandle<(), (SharedMut<Environment>, &UiResult)> for RenderAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (shared_environment, ui_result): (SharedMut<Environment>, &UiResult),
    ) -> Result<(), Error> {
        let environment = shared_environment.read();
        let screen: RenderTarget<'_> = frame_input.screen();

        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        screen.write(|| {
            self.render(&environment);
            ui_result.render();
        });

        Ok(())
    }
}

impl Adapter<(), (SharedMut<Environment>, &UiAdapter), RenderEvent> for RenderAdapter {
    fn from_context(context: &Context) -> (EventWriter<RenderEvent>, Self) {
        let (reader, writer) = create_event_bundle::<RenderEvent>();

        (
            writer,
            Self {
                shared_state: RenderState {
                    workpiece: SharedMut::default(),
                },
                event_reader: reader,
            },
        )
    }

    fn get_reader(&self) -> &EventReader<RenderEvent> {
        &self.event_reader
    }

    fn handle_event(&mut self, event: RenderEvent) {}

    fn get_adapter_description(&self) -> String {
        "RenderAdapter".to_string()
    }
}
