pub type Vertices = Vec<glam::Vec3>;

pub trait Mesh {
    fn to_vertices(&self) -> Vertices;
    fn to_vertices_flipped(&self) -> Vertices;
}

pub trait WithOffset {
    fn with_offset(&self, offset: glam::Vec3) -> Self;
}

impl WithOffset for Vertices {
    fn with_offset(&self, offset: glam::Vec3) -> Self {
        self.iter().map(|v| *v + offset).collect()
    }
}

#[derive(Debug)]
pub struct MeshRef {
    start: usize,
    end: usize,
}

impl MeshRef {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn slice_from_positions<'a>(&self, vec: &'a [[f32; 3]]) -> &'a [[f32; 3]] {
        let start = self.start;
        let end = self.end;

        &vec[start..end]
    }

    pub fn slice_from_colors<'a>(&self, vec: &'a [[u8; 4]]) -> &'a [[u8; 4]] {
        let start = self.start;
        let end = self.end;

        &vec[start..end]
    }
}
