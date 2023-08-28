use three_d::Vector3;
use three_d_asset::{vec3, Indices, InnerSpace, Positions, Srgba, TriMesh};

use super::{instruction::InstructionType, movement, state::State, GCode};

#[derive(Debug, Clone)]
pub struct PathLine {
    start: Vector3<f64>,
    end: Vector3<f64>,
    print: bool,
}

impl PathLine {
    pub fn direction(&self) -> Vector3<f64> {
        self.end - self.start
    }

    pub fn flip_yz(&self) -> Self {
        Self {
            start: vec3(self.start.x, self.start.z, self.start.y),
            end: vec3(self.end.x, self.end.z, self.end.y),
            print: self.print,
        }
    }
}

pub struct PathModul {
    points: Vec<PathLine>,
    state: State,
}

impl PathModul {
    pub fn new(points: Vec<PathLine>, state: super::state::State) -> Self {
        Self { points, state }
    }

    pub fn points(&self) -> &Vec<PathLine> {
        &self.points
    }

    pub fn state(&self) -> &super::state::State {
        &self.state
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

    pub fn add_line_with_fields(&mut self, points: Vec<PathLine>, state: super::state::State) {
        self.path.push(PathModul { points, state });
    }

    pub fn path(&self) -> &Vec<PathModul> {
        &self.path
    }
}

impl From<GCode> for ToolPath {
    fn from(value: GCode) -> Self {
        let mut tool_path = ToolPath::new();

        let mut current_movements = movement::Movements::new();

        for instruction_modul in value.instructions() {
            let mut points = Vec::new();
            let state = instruction_modul.gcode_state();

            for instruction in instruction_modul.instructions() {
                let movements = instruction.movements();
                let last_point = current_movements.to_vec3(vec3(0.0, 0.0, 0.0));

                current_movements.add_movements(movements);

                let current_point = current_movements.to_vec3(vec3(0.0, 0.0, 0.0));

                let print = instruction.instruction_type() == &InstructionType::G1
                    && current_movements.E.is_some_and(|e| e > 0.0);

                points.push(PathLine {
                    start: last_point,
                    end: current_point,
                    print,
                });
            }

            tool_path.add_line(PathModul::new(points, state.clone()));
        }

        tool_path
    }
}

#[allow(dead_code)]
#[derive(Debug)]
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

    pub fn mesh(&self) -> &TriMesh {
        &self.mesh
    }

    pub fn color(&self) -> &Srgba {
        &self.color
    }
}

trait FlipYZ {
    fn flip_yz(&mut self);
}

impl FlipYZ for Vector3<f64> {
    fn flip_yz(&mut self) {
        std::mem::swap(&mut self.y, &mut self.z);
    }
}

#[allow(dead_code)]
impl From<&PathModul> for PathModulMesh {
    fn from(path_modul: &PathModul) -> Self {
        let diameter = 0.45;
        let mut last_cross: Option<Cross> = None;
        let mut positions = Vec::new();

        for element in path_modul.points.iter().enumerate() {
            let path = element.1;

            if path.print {
                let direction = path.direction();
                let cross = get_cross(direction, diameter / 2.0);

                if let Some(last) = last_cross.take() {
                    draw_cross_connection(&path.start, &cross, &last, &mut positions);
                } else {
                    draw_rect_with_cross(&path.start, &cross, &mut positions);
                }

                draw_path(path, &mut positions, &cross);
                last_cross = Some(cross);
            } else if let Some(last) = last_cross.take() {
                draw_rect_with_cross(&path.start, &last, &mut positions);
            }

            if element.0 == path_modul.points.len() - 1 {
                if let Some(last) = last_cross.take() {
                    draw_rect_with_cross(&path.end, &last, &mut positions);
                }
            }
        }

        let indices: Vec<u32> = (0..positions.len() as u32).collect();

        for position in positions.iter_mut() {
            position.flip_yz();
        }

        let mut mesh = TriMesh {
            positions: Positions::F64(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        };

        mesh.compute_normals();

        Self {
            mesh,
            color: path_modul
                .state
                .print_type
                .as_ref()
                .unwrap_or(&crate::slicer::print_type::PrintType::Unknown)
                .get_color(),
        }
    }
}

fn draw_path(path: &PathLine, positions: &mut Vec<Vector3<f64>>, cross: &Cross) {
    draw_rect(
        cross.up + path.start,
        cross.right + path.start,
        cross.up + path.end,
        cross.right + path.end,
        positions,
    );

    draw_rect(
        cross.down + path.start,
        cross.right + path.start,
        cross.down + path.end,
        cross.right + path.end,
        positions,
    );

    draw_rect(
        cross.down + path.start,
        cross.left + path.start,
        cross.down + path.end,
        cross.left + path.end,
        positions,
    );

    draw_rect(
        cross.up + path.start,
        cross.left + path.start,
        cross.up + path.end,
        cross.left + path.end,
        positions,
    );
}

fn draw_cross_connection(
    center: &Vector3<f64>,
    start_cross: &Cross,
    end_cross: &Cross,
    positions: &mut Vec<Vector3<f64>>,
) {
    //top
    positions.push(end_cross.up + center);
    positions.push(end_cross.right + center);
    positions.push(start_cross.right + center);

    positions.push(end_cross.up + center);
    positions.push(end_cross.left + center);
    positions.push(start_cross.left + center);

    //bottom
    positions.push(end_cross.down + center);
    positions.push(end_cross.right + center);
    positions.push(start_cross.right + center);

    positions.push(end_cross.down + center);
    positions.push(end_cross.left + center);
    positions.push(start_cross.left + center);
}

fn draw_rect(
    point_left_0: Vector3<f64>,
    point_left_1: Vector3<f64>,
    point_right_0: Vector3<f64>,
    point_right_1: Vector3<f64>,
    positions: &mut Vec<Vector3<f64>>,
) {
    positions.push(point_left_0);
    positions.push(point_left_1);
    positions.push(point_right_0);

    positions.push(point_right_0);
    positions.push(point_right_1);
    positions.push(point_left_1);
}

fn draw_rect_with_cross(center: &Vector3<f64>, cross: &Cross, positions: &mut Vec<Vector3<f64>>) {
    draw_rect(
        cross.up + center,
        cross.left + center,
        cross.right + center,
        cross.down + center,
        positions,
    )
}

#[derive(Debug)]
struct Cross {
    up: Vector3<f64>,
    down: Vector3<f64>,
    left: Vector3<f64>,
    right: Vector3<f64>,
}

fn get_cross(direction: Vector3<f64>, radius: f64) -> Cross {
    let horizontal = direction.cross(vec3(0.0, 0.0, direction.z + 1.0));
    let vertical = direction.cross(vec3(direction.x + 1.0, direction.y + 1.0, 0.0));

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
