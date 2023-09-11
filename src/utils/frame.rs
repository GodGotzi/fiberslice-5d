use std::cell::Ref;

use three_d::FrameInput;

use crate::application::Application;

pub trait FrameHandle {
    fn frame(&mut self, input: &FrameInput, application: Ref<'_, Application>);
}
