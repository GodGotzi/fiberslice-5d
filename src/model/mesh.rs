#[derive(Debug)]
pub struct CpuMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
}

impl CpuMesh {
    pub fn push_position(&mut self, position: [f32; 3]) {
        self.positions.push(position);
    }

    pub fn push_color(&mut self, color: [f32; 4]) {
        self.colors.push(color);
    }

    pub fn push_normal(&mut self, normal: [f32; 3]) {
        self.normals.push(normal);
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
