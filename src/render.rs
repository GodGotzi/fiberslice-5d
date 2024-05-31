use egui_glow::Painter;
use std::{cell::RefCell, collections::HashMap, ops::Deref};
use strum_macros::Display;
use three_d::{ClearState, Context, FrameInput, Gm, Mesh, Object, PhysicalMaterial, RenderTarget};
use three_d_asset::{vec3, Mat4, Positions, TriMesh};

use crate::{
    environment::Environment,
    event::{create_event_bundle, EventReader, EventWriter},
    model::{gcode::PrintPart, mesh::ToFlipYZ},
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
    context: Context,
    shared_state: RenderState,
    components: HashMap<String, Gm<Mesh, PhysicalMaterial>>,

    ui_painter: RefCell<Painter>,
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
        let now = std::time::Instant::now();

        let read = self.shared_state.workpiece.read();
        let workpiece = read.as_ref().unwrap();

        let mut center_mass = workpiece.center_mass;

        println!("Center mass: {:?}", center_mass);
        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        for (_, layer) in workpiece.layers.iter() {
            for modul in layer.iter() {
                vertices.extend(modul.mesh.flip_yz());
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

        println!("Update took {:?}", now.elapsed());

        drop(read);

        let mut cpu_mesh = TriMesh {
            positions: Positions::F32(vertices),
            colors: Some(colors),
            ..Default::default()
        };

        cpu_mesh.compute_normals();

        println!("Update took {:?}", now.elapsed());

        let mesh = Mesh::new(&self.context, &cpu_mesh);

        let mut model = Gm::new(mesh, PhysicalMaterial::default());

        std::mem::swap(&mut center_mass.y, &mut center_mass.z);

        model.set_transformation(Mat4::from_translation(vec3(
            -center_mass.x,
            -center_mass.y,
            -center_mass.z,
        )));

        self.components.insert("WORKPIECE".to_string(), model);

        println!("Update took {:?}", now.elapsed());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    pub fn render(&mut self, environment: &Environment) {
        for component in self.components.values() {
            component.render(environment.camera(), &environment.lights())
        }
    }
}

#[derive(Debug)]
struct RenderError {
    message: String,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderError: {}", self.message)
    }
}

impl std::error::Error for RenderError {}

impl FrameHandle<(), (SharedMut<Environment>, &Result<ParallelUiOutput, Error>)> for RenderAdapter {
    fn handle_frame(
        &mut self,
        frame_input: &FrameInput,
        (shared_environment, output): (SharedMut<Environment>, &Result<ParallelUiOutput, Error>),
    ) -> Result<(), Error> {
        let now = std::time::Instant::now();
        let environment = shared_environment.read();
        let screen: RenderTarget<'_> = frame_input.screen();

        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        screen
            .write(|| {
                self.render(&environment);

                if let Ok(output) = output {
                    // render ui
                    // println!("rendering ui");
                    output.render(&self.ui_painter);
                } else {
                    println!("not rendering ui");
                }

                Result::<(), RenderError>::Ok(())
            })
            .unwrap();

        println!("Render took {:?}", now.elapsed());

        Ok(())
    }
}

impl Adapter<(), (SharedMut<Environment>, &Result<ParallelUiOutput, Error>), RenderEvent>
    for RenderAdapter
{
    fn from_context(context: &Context) -> (EventWriter<RenderEvent>, Self) {
        let (reader, writer) = create_event_bundle::<RenderEvent>();

        let components = HashMap::new();

        (
            writer,
            Self {
                context: context.clone(),
                shared_state: RenderState {
                    workpiece: SharedMut::default(),
                },
                ui_painter: RefCell::new(Painter::new(context.deref().clone(), "", None).unwrap()),
                components,
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

impl Drop for RenderAdapter {
    fn drop(&mut self) {
        self.ui_painter.borrow_mut().destroy();
    }
}
