use three_d::Vector3;

pub struct PathLine {
    start: Vector3<f64>,
    end: Vector3<f64>,
    print: bool,
}

pub struct ToolPath {
    path: Vec<PathLine>,
}

impl ToolPath {
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    pub fn add_line(&mut self, start: Vector3<f64>, end: Vector3<f64>, print: bool) {
        self.path.push(PathLine { start, end, print });
    }

    pub fn path(&self) -> &Vec<PathLine> {
        &self.path
    }
}

/*
impl From<ToolPath> for CpuMesh {
    fn from(value: ToolPath) -> Self {


    }
}
*/
