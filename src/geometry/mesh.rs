use glam::{Vec3, Vec4};
use rether::vertex::Vertex;

pub trait Mesh<const V: usize> {
    fn to_triangle_vertices(&self) -> [Vertex; V];

    fn to_triangle_vertices_flipped(&self) -> [Vertex; V] {
        panic!("Not implemented")
    }
}

pub fn construct_triangle_vertices<const T: usize>(raw: [Vec3; T], color: Vec4) -> [Vertex; T] {
    let mut vertices = [Vertex::default(); T];
    let color = color.to_array();

    for i in (0..vertices.len()).step_by(3) {
        let v0 = raw[i];
        let v1 = raw[i + 1];
        let v2 = raw[i + 2];

        let normal = (v1 - v0).cross(v2 - v0).normalize();

        vertices[i].position = v0.to_array();
        vertices[i + 1].position = v1.to_array();
        vertices[i + 2].position = v2.to_array();

        vertices[i].color = color;
        vertices[i + 1].color = color;
        vertices[i + 2].color = color;

        vertices[i].normal = normal.to_array();
        vertices[i + 1].normal = normal.to_array();
        vertices[i + 2].normal = normal.to_array();
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
