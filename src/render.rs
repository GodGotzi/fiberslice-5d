use std::{cell::RefCell, ops::Deref};

use egui_glow::Painter;
use three_d::{ClearState, Context, FrameInput, RenderTarget};

use crate::{
    environment::Environment,
    event::{create_event_bundle, EventReader, EventWriter},
    model::gcode::PrintPart,
    prelude::*,
    ui::parallel::ParallelUiOutput,
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

    ui_painter: RefCell<Painter>,
    event_reader: EventReader<RenderEvent>,
}

impl RenderAdapter {
    pub fn share_state(&self) -> RenderState {
        self.shared_state.clone()
    }

    pub fn render(&mut self, environment: &Environment) {

        //.render(environment.camera(), environment.lights().as_slice());
    }
}

impl FrameHandle<(), (SharedMut<Environment>, &Result<ParallelUiOutput, Error>)> for RenderAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (shared_environment, output): (SharedMut<Environment>, &Result<ParallelUiOutput, Error>),
    ) -> Result<(), Error> {
        let environment = shared_environment.read();
        let screen: RenderTarget<'_> = frame_input.screen();

        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        screen.write(|| {
            self.render(&environment);

            if let Ok(output) = output {
                //render ui
                println!("rendering ui");
                output.render(&self.ui_painter);
            }
        });

        Ok(())
    }
}

impl Adapter<(), (SharedMut<Environment>, &Result<ParallelUiOutput, Error>), RenderEvent>
    for RenderAdapter
{
    fn from_context(context: &Context) -> (EventWriter<RenderEvent>, Self) {
        let (reader, writer) = create_event_bundle::<RenderEvent>();

        (
            writer,
            Self {
                shared_state: RenderState {
                    workpiece: SharedMut::default(),
                },
                ui_painter: RefCell::new(Painter::new(context.deref().clone(), "", None).unwrap()),
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
