pub type Vertices = Vec<glam::Vec3>;

pub trait ToFlipYZ {
    fn flip_yz(&self) -> Self;
}

impl ToFlipYZ for Vertices {
    fn flip_yz(&self) -> Self {
        let mut vertices = self.clone();

        for vertex in vertices.iter_mut() {
            std::mem::swap(&mut vertex.y, &mut vertex.z);
        }

        vertices
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
