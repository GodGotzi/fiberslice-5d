use three_d::{ClearState, Context, FrameInput, Gm, Mesh, PhysicalMaterial, RenderTarget, GUI};

use crate::{
    environment::Environment,
    event::{create_event_bundle, EventReader, EventWriter},
    prelude::*,
};

#[derive(Debug)]
pub enum RenderEvent {}

pub struct RenderState {
    toolpath: Option<Gm<Mesh, PhysicalMaterial>>,
}

pub struct RenderAdapter {
    shared_state: SharedMut<RenderState>,
    event_reader: EventReader<RenderEvent>,
}

impl RenderAdapter {
    pub fn share_state(&self) -> SharedMut<RenderState> {
        self.shared_state.clone()
    }

    pub fn set_toolpath(&mut self, toolpath: Gm<Mesh, PhysicalMaterial>) {
        self.shared_state.lock_expect().toolpath = Some(toolpath);
    }

    pub fn render(&mut self, environment: &Environment) {
        let mut state = self.shared_state.lock_expect();

        /*
               state
           .toolpath
           .as_mut()
           .unwrap()
           .render(environment.camera(), environment.lights().as_slice());
        */
    }
}

impl FrameHandle<(), (SharedMut<Environment>, &GUI)> for RenderAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (shared_environment, gui): (SharedMut<Environment>, &GUI),
    ) -> Result<(), Error> {
        let environment = shared_environment.lock_expect();

        let screen: RenderTarget<'_> = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));

        screen.write(|| {
            self.render(&environment);
            gui.render();
        });

        Ok(())
    }
}

impl Adapter<(), (SharedMut<Environment>, &GUI), RenderEvent> for RenderAdapter {
    fn from_context(context: &Context) -> (EventWriter<RenderEvent>, Self) {
        let (reader, writer) = create_event_bundle::<RenderEvent>();

        (
            writer,
            Self {
                shared_state: SharedMut::from_inner(RenderState { toolpath: None }),
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
