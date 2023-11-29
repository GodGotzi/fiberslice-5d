use three_d::{Srgba, Vector3};

#[derive(Debug)]
pub struct SimpleMesh {
    pub positions: Vec<Vector3<f32>>,
    pub colors: Vec<Srgba>,
}

impl SimpleMesh {
    pub fn push_position(&mut self, position: Vector3<f32>) {
        self.positions.push(position);
    }

    pub fn push_color(&mut self, color: Srgba) {
        self.colors.push(color);
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

    pub fn slice_from_vec<'a>(&self, vec: &'a [[f32; 3]]) -> &'a [[f32; 3]] {
        let start = self.start;
        let end = self.end;

        &vec[start..end]
    }
}
