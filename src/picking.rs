use crate::{
    prelude::{Adapter, Error, FrameHandle, WgpuContext},
    GlobalState, RootEvent,
};

#[derive(Debug, Clone)]
pub enum PickingEvent {
    Select,
}

pub struct PickingState {}

pub struct PickingAdapter {}

impl FrameHandle<'_, RootEvent, (), GlobalState<RootEvent>> for PickingAdapter {
    fn handle_frame(
        &mut self,
        _event: &winit::event::Event<RootEvent>,
        _start_time: std::time::Instant,
        _wgpu_context: &WgpuContext,
        _context: GlobalState<RootEvent>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> Adapter<'a, RootEvent, PickingState, (), GlobalState<RootEvent>, PickingEvent>
    for PickingAdapter
{
    fn from_context(context: &WgpuContext) -> (PickingState, Self) {
        (PickingState {}, Self {})
    }

    fn get_adapter_description(&self) -> String {
        "PickingAdapter".to_string()
    }
}
