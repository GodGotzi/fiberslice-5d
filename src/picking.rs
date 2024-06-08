use crate::{
    prelude::{event::*, Adapter, Error, FrameHandle, WgpuContext},
    GlobalState,
};

#[derive(Debug, Clone)]
pub enum PickingEvent {
    Select,
}

pub struct PickingAdapter {}

impl FrameHandle<(), (), GlobalState> for PickingAdapter {
    fn handle_frame(
        &mut self,
        _event: &winit::event::Event<()>,
        _start_time: std::time::Instant,
        _wgpu_context: &WgpuContext,
        _context: GlobalState,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl Adapter<(), (), GlobalState, PickingEvent> for PickingAdapter {
    fn from_context(context: &WgpuContext) -> Self {
        Self {}
    }

    fn handle_event(&mut self, event: PickingEvent) {}

    fn get_adapter_description(&self) -> String {
        "PickingAdapter".to_string()
    }
}
