use three_d::Context;

use crate::{
    event::EventReader,
    prelude::{Adapter, Error, FrameHandle, SharedMut},
    render::RenderState,
};

#[derive(Debug)]
pub enum PickingEvent {}

pub struct PickingAdapter {
    event_reader: EventReader<PickingEvent>,
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

impl Adapter<(), SharedMut<RenderState>, PickingEvent> for PickingAdapter {
    fn from_context(context: &Context) -> (crate::event::EventWriter<PickingEvent>, Self) {
        let (reader, writer) = crate::event::create_event_bundle::<PickingEvent>();

        (
            writer,
            Self {
                event_reader: reader,
            },
        )
    }

    fn get_reader(&self) -> &EventReader<PickingEvent> {
        &self.event_reader
    }

    fn handle_event(&mut self, event: PickingEvent) {}

    fn get_adapter_description(&self) -> String {
        "PickingAdapter".to_string()
    }
}
