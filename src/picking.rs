use three_d::Context;

use crate::{
    prelude::{Adapter, Error, FrameHandle, SharedMut},
    render::RenderState,
};

pub struct PickingAdapter {}

impl PickingAdapter {
    pub fn from_context(_context: &Context) -> Self {
        Self {}
    }
}

impl FrameHandle<(), SharedMut<RenderState>> for PickingAdapter {
    fn handle_frame(
        &mut self,
        _frame_input: &three_d::FrameInput,
        _state: SharedMut<RenderState>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl Adapter<(), SharedMut<RenderState>> for PickingAdapter {}
