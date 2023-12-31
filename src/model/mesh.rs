use three_d::Vector3;

pub type Traingle = (Vector3<f32>, Vector3<f32>, Vector3<f32>);
pub type Vertices = Vec<Vector3<f32>>;

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
