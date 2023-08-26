use three_d::Vector3;
use three_d_asset::{vec3, Indices, InnerSpace, Positions, Srgba, TriMesh};

pub struct PathModul {
    points: Vec<Vector3<f64>>,
    state: super::state::State,
    print: bool,
}

impl PathModul {
    pub fn new(points: Vec<Vector3<f64>>, state: super::state::State, print: bool) -> Self {
        Self {
            points,
            state,
            print,
        }
    }

    pub fn points(&self) -> &Vec<Vector3<f64>> {
        &self.points
    }

    pub fn state(&self) -> &super::state::State {
        &self.state
    }

    pub fn print(&self) -> bool {
        self.print
    }
}

#[derive(Default)]
pub struct ToolPath {
    path: Vec<PathModul>,
}

impl ToolPath {
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    pub fn add_line(&mut self, path_modul: PathModul) {
        self.path.push(path_modul);
    }

    pub fn add_line_with_fields(
        &mut self,
        points: Vec<Vector3<f64>>,
        state: super::state::State,
        print: bool,
    ) {
        self.path.push(PathModul {
            points,
            state,
            print,
        });
    }

    pub fn path(&self) -> &Vec<PathModul> {
        &self.path
    }
}

#[allow(dead_code)]
pub struct PathModulMesh {
    mesh: TriMesh,
    color: Srgba,
}

#[allow(dead_code)]
impl PathModulMesh {
    pub fn new(mesh: TriMesh, color: Srgba) -> Self {
        Self { mesh, color }
    }

    pub fn set_color(&mut self, color: Srgba) {
        self.color = color;
    }
}

#[allow(dead_code)]
impl From<&PathModul> for PathModulMesh {
    fn from(path_modul: &PathModul) -> Self {
        let diameter = 0.2;
        let mut last_point = path_modul.points[0];

        let mut positions = Vec::new();

        for point in path_modul.points.iter().enumerate() {
            let direction = point.1 - last_point;

            let cross = get_cross(direction, diameter / 2.0);
        }

        let indices: Vec<u32> = (0..positions.len() as u32).collect();

        let mut mesh = TriMesh {
            positions: Positions::F64(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        };

        mesh.compute_normals();

        Self {
            mesh,
            color: Srgba::new(0, 0, 0, 1),
        }
    }
}

struct Cross {
    up: Vector3<f64>,
    down: Vector3<f64>,
    left: Vector3<f64>,
    right: Vector3<f64>,
}

fn get_cross(direction: Vector3<f64>, radius: f64) -> Cross {
    let horizontal = direction.cross(vec3(0.0, 0.0, direction.z));
    let vertical = direction.cross(vec3(direction.x, 0.0, 0.0));

    Cross {
        up: vertical.normalize() * radius,
        down: vertical.normalize() * (-radius),
        left: horizontal.normalize() * radius,
        right: horizontal.normalize() * (-radius),
    }
}

impl From<ToolPath> for Vec<PathModulMesh> {
    fn from(tool_path: ToolPath) -> Self {
        let mut meshes = Vec::new();

        for path_modul in tool_path.path.iter() {
            let mesh = PathModulMesh::from(path_modul);

            meshes.push(mesh);
        }

        meshes
    }
}
