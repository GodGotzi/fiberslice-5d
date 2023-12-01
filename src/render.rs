use three_d::{ClearState, Context, FrameInput, RenderTarget, GUI};

use crate::{environment::Environment, prelude::*};

pub struct RenderState {}

pub struct RenderAdapter {
    shared_state: SharedMut<RenderState>,
}

impl RenderAdapter {
    pub fn from_context(context: &Context) -> Self {
        Self {
            shared_state: SharedMut::from_inner(RenderState {}),
        }
    }

    pub fn share_state(&self) -> SharedMut<RenderState> {
        self.shared_state.clone()
    }
}

impl FrameHandle<(), (SharedMut<Environment>, &GUI)> for RenderAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (shared_environment, gui): (SharedMut<Environment>, &GUI),
    ) -> Result<(), Error> {
        let mut environment = shared_environment.lock_expect();

        let screen: RenderTarget<'_> = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));

        screen.write(|| {
            gui.render();
        });

        Ok(())
    }
}

impl Adapter<(), (SharedMut<Environment>, &GUI)> for RenderAdapter {}
