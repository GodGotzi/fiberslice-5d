use std::collections::HashMap;

use three_d::{
    ClearState, Context, FrameInput, Gm, Mesh, Object, PhysicalMaterial, RenderTarget, GUI,
};
use three_d_asset::{vec3, Mat4, Positions, Srgba, TriMesh};

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
    context: Context,
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

        let center_mass = workpiece.center_mass;

        println!("Center mass: {:?}", center_mass);
        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        for (_, layer) in workpiece.layers.iter() {
            for modul in layer.iter() {
                vertices.extend(modul.mesh.clone());
                colors.extend(vec![
                    modul
                        .state
                        .print_type
                        .as_ref()
                        .unwrap_or(&crate::slicer::print_type::PrintType::Unknown)
                        .get_color();
                    modul.mesh.len()
                ]);
            }
        }

        drop(read);

        let mut cpu_mesh = TriMesh {
            positions: Positions::F32(vertices),
            colors: Some(colors),
            ..Default::default()
        };

        cpu_mesh.compute_normals();

        let mesh = Mesh::new(&self.context, &cpu_mesh);

        let mut model = Gm::new(mesh, PhysicalMaterial::default());

        model.set_transformation(Mat4::from_translation(vec3(
            -center_mass.x,
            -center_mass.y,
            -center_mass.z,
        )));

        self.components.insert("WORKPIECE".to_string(), model);
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
                context: context.clone(),
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
