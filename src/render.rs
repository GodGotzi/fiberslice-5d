use std::collections::HashMap;

use three_d::{
    ClearState, Context, FrameInput, Gm, Mesh, Object, PhysicalMaterial, RenderTarget, GUI,
};

use crate::{
    environment::Environment,
    event::{create_event_bundle, EventReader, EventWriter},
    model::gcode::PrintPart,
    prelude::*,
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

    components: HashMap<String, Gm<Mesh, PhysicalMaterial>>,
    event_reader: EventReader<RenderEvent>,
}

impl RenderAdapter {
    pub fn share_state(&self) -> RenderState {
        self.shared_state.clone()
    }

    pub fn set_workpiece(&mut self, workpiece: PrintPart) {
        self.shared_state.workpiece.write().replace(workpiece);
    }

    pub fn update_from_state(&mut self) {
        let read = self.shared_state.workpiece.read();
        let workpiece = read.as_ref().unwrap();

        let mut vertices = Vec::new();

        for (_, layer) in workpiece.layers.iter() {
            for modul in layer.iter() {
                vertices.extend(modul.mesh.clone());
            }
        }

        let cpu_mesh = TriMesh {
            
        }
    }

    pub fn render(&mut self, environment: &Environment) {
        for component in self.components.values() {
            component.render(environment.camera(), &environment.lights())
        }
    }
}

impl FrameHandle<(), (SharedMut<Environment>, &GUI)> for RenderAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (shared_environment, gui): (SharedMut<Environment>, &GUI),
    ) -> Result<(), Error> {
        let environment = shared_environment.read();

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
                shared_state: RenderState {
                    workpiece: SharedMut::default(),
                },

                components: HashMap::new(),
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
