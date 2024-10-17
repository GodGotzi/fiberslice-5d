use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use glam::{vec3, Mat4, Vec3};

use crate::IndexedTriangle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectVertex(Vec3);

impl ObjectVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}

impl Deref for ObjectVertex {
    type Target = Vec3;

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

impl std::ops::Mul<ObjectVertex> for Mat4 {
    type Output = ObjectVertex;

    fn mul(self, vertex: ObjectVertex) -> ObjectVertex {
        ObjectVertex(self.transform_point3(vertex.0))
    }
}

#[derive(Debug, Clone)]
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

    pub fn min_max(&self) -> (Vec3, Vec3) {
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for vertex in &self.vertices {
            min = min.min(vertex.0);
            max = max.max(vertex.0);
        }

        (min, max)
    }

    pub fn transform(&mut self, transform: Mat4) {
        *self = transform * self.clone();
    }

    pub fn sort_indices(&mut self) {
        self.triangles.iter_mut().for_each(|triangle| {
            let v0 = self.vertices[triangle[0]];
            let v1 = self.vertices[triangle[1]];
            let v2 = self.vertices[triangle[2]];

            if v0 < v1 && v0 < v2 {
                *triangle = IndexedTriangle([triangle[0], triangle[1], triangle[2]])
            } else if v1 < v0 && v1 < v2 {
                *triangle = IndexedTriangle([triangle[1], triangle[2], triangle[0]])
            } else {
                *triangle = IndexedTriangle([triangle[2], triangle[0], triangle[1]])
            }
        });
    }
}

impl std::ops::Mul<ObjectMesh> for Mat4 {
    type Output = ObjectMesh;

    fn mul(self, mesh: ObjectMesh) -> ObjectMesh {
        let vertices = mesh
            .vertices
            .into_iter()
            .map(|vertex| self * vertex)
            .collect();

        ObjectMesh {
            vertices,
            triangles: mesh.triangles,
        }
    }
}

impl From<nom_stl::Mesh> for ObjectMesh {
    fn from(mesh: nom_stl::Mesh) -> Self {
        let indexed: nom_stl::IndexMesh = mesh.into();

        let vertices: Vec<ObjectVertex> = indexed
            .vertices()
            .into_iter()
            .map(|vertex| ObjectVertex(vec3(vertex[0], vertex[1], vertex[2])))
            .collect();

        let triangles: Vec<IndexedTriangle> = indexed
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
