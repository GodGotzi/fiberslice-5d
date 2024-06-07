use crate::{
    event::EventReader,
    prelude::{Adapter, Error, FrameHandle, WgpuContext},
    render::RenderState,
};

#[derive(Debug, Clone)]
pub enum PickingEvent {
    Select,
}

pub struct PickingAdapter {
    event_reader: EventReader<PickingEvent>,
}

impl FrameHandle<(), (), RenderState> for PickingAdapter {
    fn handle_frame(
        &mut self,
        _event: &winit::event::Event<()>,
        _start_time: std::time::Instant,
        _wgpu_context: &WgpuContext,
        _context: RenderState,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl Adapter<(), (), RenderState, PickingEvent> for PickingAdapter {
    fn from_context(context: &WgpuContext) -> (crate::event::EventWriter<PickingEvent>, Self) {
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
