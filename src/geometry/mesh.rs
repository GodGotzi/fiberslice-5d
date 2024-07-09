use glam::{Vec3, Vec4};

use crate::render::vertex::Vertex;

pub trait Mesh<const V: usize> {
    fn to_triangle_vertices(&self) -> [Vertex; V];

    fn to_triangle_vertices_flipped(&self) -> [Vertex; V] {
        panic!("Not implemented")
    }
}

pub fn construct_triangle(a: Vec3, b: Vec3, c: Vec3, color: Vec4) -> [Vertex; 3] {
    let normal = (-(b - a).cross(c - a).normalize()).to_array();
    let color = color.to_array();

    [
        Vertex {
            position: a.to_array(),
            color,
            normal,
        },
        Vertex {
            position: b.to_array(),
            color,
            normal,
        },
        Vertex {
            position: c.to_array(),
            color,
            normal,
        },
    ]
}

pub fn construct_triangle_vertices<const T: usize>(raw: [Vec3; T], color: Vec4) -> [Vertex; T] {
    let mut vertices = [Vertex::default(); T];

    for index in (0..vertices.len()).step_by(3) {
        let triangle = construct_triangle(raw[index], raw[index + 1], raw[index + 2], color);

        vertices[index] = triangle[0];
        vertices[index + 1] = triangle[1];
        vertices[index + 2] = triangle[2];
    }

    vertices
}

pub trait IndexedMesh<const V: usize, const I: usize>: Mesh<V> {
    fn to_triangle_indices(&self) -> [u32; I];
    fn to_triangle_indices_flipped(&self) -> [u32; I] {
        panic!("Not implemented")
    }
}

pub trait WireMesh<const V: usize> {
    fn to_wire_vertices(&self) -> [Vertex; V];
}

pub fn construct_wire_vertices<const T: usize>(raw: [Vec3; T], color: Vec4) -> [Vertex; T] {
    let mut vertices = [Vertex::default(); T];

    for index in 0..vertices.len() {
        let color = color.to_array();

        vertices[index] = Vertex {
            position: raw[index].to_array(),
            color,
            normal: [0.0, 0.0, 0.0],
        };
    }

    vertices
}

pub trait IndexedWireMesh<const V: usize, const I: usize>: WireMesh<V> {
    fn to_wire_indices(&self) -> [u32; I];
}
