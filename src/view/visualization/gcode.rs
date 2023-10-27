use std::cell::RefCell;
use std::collections::HashMap;

use bevy::prelude::Mesh;
use bevy::render::render_resource::PrimitiveTopology;

use crate::model::gcode::toolpath::*;
use crate::model::gcode::GCode;
use crate::model::mesh::*;

pub fn create_toolpath(gcode: GCode) -> ToolPathModel {
    let toolpath = ToolPath::from(gcode.clone());
    let modul_map: HashMap<usize, Vec<PathModul>> = toolpath.into();

    let mut layers: HashMap<usize, RefCell<LayerMesh>> = HashMap::new();

    for entry in modul_map.iter() {
        let layer = LayerMesh::empty();
        layers.insert(*entry.0, RefCell::new(layer));
    }

    unsafe {
        for entry in modul_map.into_iter() {
            let layer = layers.get(&entry.0).unwrap();
            let coordinator = PartCoordinator::new(layer.as_ptr().as_mut().unwrap());

            for modul in entry.1 {
                coordinator.compute_model(&modul);
                coordinator.finish();
            }
        }
    }

    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut normals = Vec::new();
    let mut mesh_models = HashMap::new();

    for entry in layers.iter() {
        let mut layer = entry.1.borrow_mut();

        positions.append(&mut layer.mesh.positions);
        colors.append(&mut layer.mesh.colors);
        normals.append(&mut layer.mesh.normals);

        mesh_models.insert(
            *entry.0,
            LayerContext {
                id: *entry.0,
                line_range: layer.line_range,
            },
        );
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    //mesh.set_indices(Some(Indices::U32(
    //    (0..colors.len()).map(|e| e as u32).collect(),
    //)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    ToolPathModel {
        mesh,
        gcode,
        layers: mesh_models,
    }
}

/*
impl GCodeVisualizer {
    pub fn try_collect_objects<'a>(
        &self,
        context: &Context,
    ) -> Result<ToolPathModel<'a>, crate::error::Error> {
        let mut toolpath_model = build_test_meshes(context);

        toolpath_model.model.set_transformation(
            Mat4::from_translation(vec3(-125.0, 5.0, 125.0))
                .concat(&Mat4::from_angle_x(degrees(-90.0))),
        );
        //model.set_transformation(Mat4::from_translation(vec3(0.0, 40.0, 0.0)));

        Ok(toolpath_model)
    }
}*/
