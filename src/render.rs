use three_d::{Context, FrameInput};

use crate::{environment::Environment, prelude::*};

pub struct RenderState {}

pub struct RenderAdapter {
    shared_environment: SharedMut<Environment>,
    shared_state: SharedMut<RenderState>,
}

impl RenderAdapter {
    pub fn share_environment(&self) -> SharedMut<Environment> {
        self.shared_environment.clone()
    }

    pub fn share_state(&self) -> SharedMut<RenderState> {
        self.shared_state.clone()
    }
}

impl FrameHandle<()> for RenderAdapter {
    fn handle_frame(&mut self, frame_input: &FrameInput) -> Result<(), Error> {
        Ok(())
    }
}

impl RenderHandle for RenderAdapter {
    fn handle(&self) {}
}

impl Adapter<()> for RenderAdapter {
    fn from_context(context: &Context) -> Self {
        Self {
            shared_environment: SharedMut::from_inner(Environment::new(context)),
            shared_state: SharedMut::from_inner(RenderState {}),
        }
    }
}
