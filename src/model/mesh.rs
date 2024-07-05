pub type Vertices = Vec<glam::Vec3>;

// TODO add const number of vertices
pub trait Mesh {
    fn to_vertices(&self) -> Vertices;
    fn to_vertices_flipped(&self) -> Vertices {
        panic!("Not implemented")
    }
}

pub trait Lines {
    fn to_lines(&self) -> Vertices;
}

pub trait WithOffset {
    fn with_offset(&self, offset: glam::Vec3) -> Self;
}

impl WithOffset for Vertices {
    fn with_offset(&self, offset: glam::Vec3) -> Self {
        self.iter().map(|v| *v + offset).collect()
    }
}
