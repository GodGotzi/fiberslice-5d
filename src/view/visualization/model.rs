use std::sync::Arc;
use std::sync::Mutex;

use three_d::*;
use three_d_asset::TriMesh;

use crate::application::Application;
use crate::utils::task::VirtualResultTask;
use crate::view::environment;

use super::Visualizer;

struct MeshWrapper(Mesh);

impl std::fmt::Debug for MeshWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mesh").finish()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Layer {
    mesh_wrap: MeshWrapper,
    color: Srgba,
}

#[allow(dead_code)]
pub struct GCodeVisualizer {
    gcode: Option<crate::model::gcode::GCode>,
    result: Option<Arc<Mutex<VirtualResultTask<Vec<Layer>>>>>,
}

#[allow(dead_code)]
impl GCodeVisualizer {
    pub fn new() -> GCodeVisualizer {
        GCodeVisualizer {
            gcode: None,
            result: None,
        }
    }

    pub fn set_gcode(&mut self, gcode: crate::model::gcode::GCode) {
        self.gcode = Some(gcode);
    }

    pub fn gcode(&self) -> &Option<crate::model::gcode::GCode> {
        &self.gcode
    }
}

impl Visualizer for GCodeVisualizer {
    fn visualize(&mut self, application: &mut Application) -> Result<(), crate::error::Error> {
        if self.gcode.is_none() {
            return Err(crate::error::Error::FieldMissing("gcode is missing".into()));
        }

        let mut result = VirtualResultTask::<Vec<Layer>>::new();

        let gcode = self.gcode.as_ref().unwrap().clone();

        result.run(Box::new(move || {
            let layers = Vec::new();

            CpuMesh::cube();

            for instruction in gcode.instructions().iter() {
                /*

                TODO




                */

                CpuMesh::cube();

                //strokes.push(Stroke { mesh_wrap: MeshWrapper(Mesh::new(context, cpu_mesh)), color: () })
            }

            layers
        }));

        self.result = Some(Arc::new(Mutex::new(result)));

        application
            .task_handler()
            .add_task(self.result.as_ref().unwrap().clone());

        Ok(())
    }

    fn render(
        &self,
        context: &three_d::WindowedContext,
        target: &RenderTarget<'_>,
        environment: &environment::Environment,
    ) -> Result<(), crate::error::Error> {
        let test_mesh = build_test_mesh();

        let model = Gm::new(
            InstancedMesh::new(context, &Instances::default(), &test_mesh),
            PhysicalMaterial::new(
                context,
                &CpuMaterial {
                    albedo: Srgba {
                        r: 128,
                        g: 128,
                        b: 128,
                        a: 255,
                    },
                    ..Default::default()
                },
            ),
        );

        target.render(
            environment.camera(),
            vec![model],
            environment.lights().as_slice(),
        );

        Ok(())
    }
}

pub fn build_test_mesh() -> CpuMesh {
    let positions = vec![
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 100.0, 0.0),
        vec3(100.0, 100.0, 0.0),
    ];

    let indices = vec![0, 1, 2];

    //let uvs = vec![vec2(0, 0), vec2(0, 1), vec2(1, 1)];

    let mut mesh = TriMesh {
        positions: Positions::F32(positions),
        //uvs: Some(uvs),
        indices: Indices::U16(indices),
        ..Default::default()
    };
    mesh.compute_normals();
    mesh.compute_tangents();
    mesh
}
