pub trait Mesh<const V: usize> {
    fn to_triangle_vertices(&self) -> [glam::Vec3; V];

    fn to_triangle_vertices_flipped(&self) -> [glam::Vec3; V] {
        panic!("Not implemented")
    }
}

pub trait IndexedMesh<const V: usize, const I: usize>: Mesh<V> {
    fn to_triangle_indices(&self) -> [u32; I];
    fn to_triangle_indices_flipped(&self) -> [u32; I] {
        panic!("Not implemented")
    }
}

pub trait WireMesh<const V: usize> {
    fn to_wire_vertices(&self) -> [glam::Vec3; V];
}

pub trait IndexedWireMesh<const V: usize, const I: usize>: WireMesh<V> {
    fn to_wire_indices(&self) -> [u32; I];
}
