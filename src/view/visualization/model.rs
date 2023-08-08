use std::sync::Arc;
use std::sync::Mutex;

use three_d::*;
use three_d_asset::TriMesh;

use crate::application::Application;
use crate::utils::debug::DebugWrapper;
use crate::utils::task::TaskWithResult;

use super::Visualizer;

struct MeshWrapper(TriMesh);

impl std::fmt::Debug for MeshWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_f = f.debug_struct("Mesh");

        debug_f.field("positions", &DebugWrapper::from(self.0.positions.to_f64()));

        if let Some(indices) = self.0.indices.to_u32() {
            debug_f.field("indices", &DebugWrapper::from(indices));
        }

        if let Some(uvs) = &self.0.uvs {
            debug_f.field("uvs", &DebugWrapper::from(uvs));
        }

        debug_f.finish()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Layer {
    mesh_wrap: MeshWrapper,
    color: Srgba,
}

#[allow(dead_code)]
impl Layer {
    fn triangle_mesh(&self) -> &TriMesh {
        &self.mesh_wrap.0
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GCodeVisualizer {
    gcode: Option<crate::model::gcode::GCode>,
    result: Option<Arc<Mutex<TaskWithResult<Vec<Layer>>>>>,
}

#[allow(dead_code)]
impl GCodeVisualizer {
    pub fn new() -> Self {
        Self::default()
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

        let mut result = TaskWithResult::<Vec<Layer>>::new();

        let gcode = self.gcode.as_ref().unwrap().clone();

        result.run(Box::new(move || {
            let layers = Vec::new();

            for _instruction in gcode.instructions().iter() {
                /*

                TODO




                */

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

    fn try_collect_objects(
        &self,
        context: &three_d::WindowedContext,
    ) -> Result<Vec<Box<dyn Object>>, crate::error::Error> {
        let test_mesh = build_test_mesh();

        let mut model: Gm<Mesh, PhysicalMaterial> = Gm::new(
            Mesh::new(context, &test_mesh),
            PhysicalMaterial::new(
                context,
                &CpuMaterial {
                    albedo: Srgba {
                        r: 100,
                        g: 100,
                        b: 190,
                        a: 255,
                    },
                    ..Default::default()
                },
            ),
        );

        model.set_transformation(Mat4::from_translation(vec3(0.0, 40.0, 0.0)));

        Ok(vec![Box::new(model)])
    }
}

pub fn build_test_mesh() -> CpuMesh {
    let positions = vec![
        // Up
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        // Down
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        // Back
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        // Front
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        // Right
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, -1.0),
        // Left
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, -1.0),
    ];
    let uvs = vec![
        // Up
        Vec2::new(0.25, 0.0),
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.25, 1.0 / 3.0),
        // Down
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.25, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        // Back
        Vec2::new(0.0, 2.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.0, 1.0 / 3.0),
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.0, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        // Front
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.75, 2.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 2.0 / 3.0),
        // Right
        Vec2::new(1.0, 2.0 / 3.0),
        Vec2::new(1.0, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 2.0 / 3.0),
        Vec2::new(1.0, 2.0 / 3.0),
        // Left
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
    ];

    let mut mesh = TriMesh {
        positions: Positions::F32(positions),
        uvs: Some(uvs),
        ..Default::default()
    };
    mesh.compute_normals();
    mesh.compute_tangents();
    mesh
}
