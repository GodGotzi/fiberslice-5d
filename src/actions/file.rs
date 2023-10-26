use std::f32::consts::PI;

use bevy::{
    app::AppExit,
    prelude::*,
    render::render_resource::Face,
    tasks::{AsyncComputeTaskPool, Task},
};

use bevy_mod_raycast::RaycastMesh;
use futures_lite::future::{self, block_on};
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};

use crate::{
    model::{gcode::toolpath::ToolPathModel, gcode::GCode},
    ui::data::UiData,
    view::{picking::RaycastSet, visualization::gcode::create_toolpath},
};

#[derive(Debug)]
pub enum FileActionResult {
    LoadGCode(ToolPathModel),
    SaveAs,
    Save,
    Exit,
}

#[derive(Debug)]
pub enum FileActionError {
    Cancelled,
    Error(String),
}

#[derive(Component)]
pub struct ComputeFileAction(Task<Result<FileActionResult, FileActionError>>);

pub(super) fn handle_tasks(
    mut commands: Commands,
    mut app_events: EventWriter<AppExit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transform_tasks: Query<(Entity, &mut ComputeFileAction)>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            if let Ok(result) = result {
                match result {
                    FileActionResult::LoadGCode(toolpath) => {
                        let transform =
                            Transform::from_rotation(Quat::from_rotation_y(-90.0 * PI / 180.0))
                                .with_translation(Vec3::new(100.0, 0.3, -125.0));

                        commands
                            .spawn(PbrBundle {
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
                            })
                            .insert(RaycastMesh::<RaycastSet>::default());
                    }
                    FileActionResult::Exit => {
                        app_events.send(AppExit);
                    }
                    _ => {}
                };

                commands.entity(entity).remove::<ComputeFileAction>();
            } else {
                let err = result.unwrap_err();

                match err {
                    FileActionError::Cancelled => {
                        println!("Load GCode File cancelled!");
                    }
                    FileActionError::Error(err) => {
                        println!("Load GCode File failed: {}", err);
                    }
                };
            }
        }
    }
}

pub fn load_gcode(data: UiData) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        let nfd = Nfd::new().unwrap();
        let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

        match result {
            DialogResult::Ok(path) => {
                let content = std::fs::read_to_string(path).unwrap();
                let gcode: GCode = content.try_into().unwrap();
                Ok(FileActionResult::LoadGCode(create_toolpath(gcode)))
            }
            DialogResult::Err(err) => Err(FileActionError::Error(err.to_string())),
            DialogResult::Cancel => Err(FileActionError::Cancelled),
        }
    });

    data.commands.borrow_mut().spawn(ComputeFileAction(task));
}

pub fn save_as_gcode(data: UiData) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move { Ok(FileActionResult::SaveAs) });

    data.commands.borrow_mut().spawn(ComputeFileAction(task));
}

pub fn save_gcode(data: UiData) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move { Ok(FileActionResult::Save) });

    data.commands.borrow_mut().spawn(ComputeFileAction(task));
}

pub fn exit(data: UiData) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move { Ok(FileActionResult::Exit) });

    data.commands.borrow_mut().spawn(ComputeFileAction(task));
}
