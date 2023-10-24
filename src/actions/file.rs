use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::Face,
    tasks::{AsyncComputeTaskPool, Task},
};

use futures_lite::future::{self, block_on};

use crate::{
    model::{gcode::GCode, layer::ToolPathModel},
    ui::data::UiData,
    view::visualization::gcode::create_toolpath,
};

pub enum FileActionResult {
    LoadGCode(ToolPathModel<'static>),
    SaveAs,
    Save,
    Export,
    Exit,
}

#[derive(Component)]
pub struct ComputeFileAction(Task<FileActionResult>);

pub(super) fn handle_tasks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transform_tasks: Query<(Entity, &mut ComputeFileAction)>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            match result {
                FileActionResult::LoadGCode(toolpath) => {
                    let transform =
                        Transform::from_rotation(Quat::from_rotation_y(-90.0 * PI / 180.0))
                            .with_translation(Vec3::new(100.0, 0.3, -125.0));

                    commands.spawn(PbrBundle {
                        mesh: meshes.add(toolpath.mesh),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                            cull_mode: Some(Face::Front),
                            reflectance: 0.01,
                            metallic: 0.0,
                            ..Default::default()
                        }),
                        transform,
                        ..Default::default()
                    });
                }
                FileActionResult::SaveAs => {}
                FileActionResult::Save => {}
                FileActionResult::Export => {}
                FileActionResult::Exit => {}
            }

            commands.entity(entity).remove::<ComputeFileAction>();
        }
    }
}

pub fn load_gcode(data: UiData) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        let content = std::fs::read_to_string("gcode/benchy.gcode").unwrap();
        let gcode: GCode = content.try_into().unwrap();
        FileActionResult::LoadGCode(create_toolpath(&gcode))
    });

    data.commands.borrow_mut().spawn(ComputeFileAction(task));
}
