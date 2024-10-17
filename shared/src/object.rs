use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use nalgebra::Vector3;

use crate::IndexedTriangle;

#[derive(Debug, PartialEq)]
pub struct ObjectVertex(Vector3<f64>);

impl Deref for ObjectVertex {
    type Target = Vector3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ObjectVertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialOrd for ObjectVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.z != other.z {
            self.z.partial_cmp(&other.z)
        } else if self.y != other.y {
            self.y.partial_cmp(&other.y)
        } else {
            self.x.partial_cmp(&other.x)
        }
    }
}

#[derive(Debug)]
pub struct ObjectMesh {
    vertices: Vec<ObjectVertex>,
    triangles: Vec<IndexedTriangle>,
}

impl ObjectMesh {
    pub fn vertices(&self) -> &[ObjectVertex] {
        &self.vertices
    }

    pub fn triangles(&self) -> &[IndexedTriangle] {
        &self.triangles
    }
}

impl From<nom_stl::Mesh> for ObjectMesh {
    fn from(mesh: nom_stl::Mesh) -> Self {
        let indexed: nom_stl::IndexMesh = mesh.into();

        let vertices = indexed
            .vertices()
            .into_iter()
            .map(|vertex| {
                ObjectVertex(Vector3::new(
                    vertex[0] as f64,
                    vertex[1] as f64,
                    vertex[2] as f64,
                ))
            })
            .collect();

        let triangles = indexed
            .triangles()
            .into_iter()
            .map(|triangle| {
                IndexedTriangle([
                    triangle.vertices_indices()[0],
                    triangle.vertices_indices()[1],
                    triangle.vertices_indices()[2],
                ])
            })
            .collect();

        Self {
            vertices,
            triangles,
        }
    }
}
