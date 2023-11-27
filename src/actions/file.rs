use std::f32::consts::PI;

use bevy::{
    app::AppExit,
    math::vec3,
    prelude::*,
    render::render_resource::Face,
    tasks::{AsyncComputeTaskPool, Task},
};

use futures_lite::future::{self, block_on};
use nfde::{DialogResult, FilterableDialogBuilder, Nfd, SingleFileDialogBuilder};

use crate::{
    model::{gcode::toolpath::ToolpathModel, gcode::GCode},
    ui::UiData,
};

#[derive(Debug)]
pub enum FileActionResult {
    LoadGCode((Mesh, ToolpathModel)),
    ImportIntersectionObject,
    SaveAs,
    Save,
    Exit,
}

#[derive(Debug)]
pub enum FileActionError {
    Cancelled,
    Error(String),
}

/*
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
                    FileActionResult::LoadGCode((toolpath_mesh, toolpath)) => {
                        let mut transform = Transform::from_translation(vec3(0.0, 0.0, 0.0))
                            .with_rotation(Quat::from_rotation_y(-PI / 2.0));

                        if let Some(center) = toolpath.center {
                            let rotated_center = transform * center;

                            transform = transform.with_translation(-rotated_center);
                        }

                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(toolpath_mesh),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                                    cull_mode: Some(Face::Back),
                                    reflectance: 0.01,
                                    metallic: 0.0,
                                    ..Default::default()
                                }),
                                transform,
                                ..Default::default()
                            },
                            toolpath,
                        ));
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

                Ok(FileActionResult::LoadGCode(gcode.into_toolpath()))
            }
            DialogResult::Err(err) => Err(FileActionError::Error(err.to_string())),
            DialogResult::Cancel => Err(FileActionError::Cancelled),
        }
    });

    data.commands.borrow_mut().spawn(ComputeFileAction(task));
}

pub fn import_intersection_object(data: UiData) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        let nfd = Nfd::new().unwrap();
        let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

        match result {
            DialogResult::Ok(_path) => Ok(FileActionResult::ImportIntersectionObject),
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
*/
