use std::{cell::Cell, collections::HashMap};
use three_d::{vec3, InnerSpace, Positions, Srgba, Vector3};

use crate::{
    api::{math::DirectMul, FlipYZ, Reverse},
    model::{
        mesh::{MeshRef, SimpleMesh},
        shapes::Rect3d,
    },
};

use super::{state::State, toolpath::PathModul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrientation {
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

#[derive(Debug)]
pub struct Cross {
    pub up: Vector3<f32>,
    pub down: Vector3<f32>,
    pub left: Vector3<f32>,
    pub right: Vector3<f32>,
}

pub fn get_cross(direction: Vector3<f32>, radius: f32) -> Cross {
    let horizontal = direction.cross(vec3(0.0, 0.0, direction.z + 1.0));
    let vertical = direction.cross(vec3(direction.x + 1.0, direction.y + 1.0, 0.0));

    Cross {
        up: vertical
            .normalize()
            .direct_mul(&vec3(radius, radius, radius)),
        down: vertical
            .normalize()
            .direct_mul(&vec3(-radius, -radius, -radius)),
        left: horizontal
            .normalize()
            .direct_mul(&vec3(radius, radius, radius)),
        right: horizontal
            .normalize()
            .direct_mul(&vec3(-radius, -radius, -radius)),
    }
}

#[derive(Debug)]
pub struct Layer {
    pub cpu_mesh: SimpleMesh,
    pub line_range: Option<(usize, usize)>,
    child_models: Vec<LayerPart>,
}

impl Layer {
    pub fn empty() -> Self {
        Self {
            cpu_mesh: SimpleMesh {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            line_range: None,
            child_models: Vec::new(),
        }
    }

    pub fn iter_children(&self) -> impl Iterator<Item = &LayerPart> {
        self.child_models.iter()
    }
}

#[derive(Debug)]
pub struct LayerPart {
    pub state: State,
    pub line_range: (usize, usize),
    main: Option<MeshRef>,
    child_meshes: Vec<MeshRef>,
}

impl LayerPart {
    pub fn new(state: State, line_range: (usize, usize)) -> Self {
        Self {
            main: None,
            state,
            line_range,
            child_meshes: Vec::new(),
        }
    }

    pub fn push_child(&mut self, child: MeshRef) {
        self.child_meshes.push(child);
    }

    pub fn get_main(&self) -> Option<&MeshRef> {
        self.main.as_ref()
    }

    pub fn iter_children(&self) -> impl Iterator<Item = &MeshRef> {
        self.child_meshes.iter()
    }
}

fn adjust_faces(direction: Vector3<f32>) -> bool {
    adjust_pane(direction.x, direction.y)
}

fn adjust_pane(x: f32, y: f32) -> bool {
    let alpha = (x / (y * y + x * x).sqrt()).asin().to_degrees();

    if (-45.0..=45.0).contains(&alpha) {
        y > 0.0
    } else {
        x < 0.0
    }
}

pub struct PartCoordinator<'a> {
    mesh: &'a mut Layer,
    offset_start: Cell<usize>,
    offset_end: Cell<usize>,
    offset_part_start: Cell<usize>,
    offset_part_end: Cell<usize>,
}

impl<'a> PartCoordinator<'a> {
    pub fn new(mesh: &'a mut Layer) -> Self {
        Self {
            mesh,
            offset_start: Cell::new(0),
            offset_end: Cell::new(0),
            offset_part_start: Cell::new(0),
            offset_part_end: Cell::new(0),
        }
    }

    pub fn add_triangle(
        &mut self,
        mut triangle: (Vector3<f32>, Vector3<f32>, Vector3<f32>),
        color: &Srgba,
    ) {
        triangle.flip();

        let mesh = &mut self.mesh.cpu_mesh;
        mesh.push_position(triangle.0);
        mesh.push_position(triangle.1);
        mesh.push_position(triangle.2);

        mesh.push_color(*color);

        self.offset_end.replace(self.offset_end.get() + 3);
        self.offset_part_end.replace(self.offset_part_end.get() + 3);
    }

    pub fn finished_child(&mut self, state: State, line_range: (usize, usize)) {
        let start = self.offset_part_start.get();
        let end = self.offset_part_end.get();

        self.offset_part_start.replace(end);

        let meshref = MeshRef::new(start, end);

        if self.mesh.child_models.last().is_none() {
            self.mesh
                .child_models
                .push(LayerPart::new(state, line_range));
        }

        self.mesh
            .child_models
            .last_mut()
            .unwrap()
            .push_child(meshref);
    }

    pub fn finish(&mut self) {
        let start = self.offset_start.get();
        let end = self.offset_end.get();

        self.offset_start.replace(end);

        let meshref = MeshRef::new(start, end);

        if let Some(last) = self.mesh.child_models.last_mut() {
            last.main = Some(meshref);
        }
    }

    pub fn compute_model(&mut self, path_modul: &PathModul) {
        let diameter = 0.45;
        let mut last_cross: Option<Cross> = None;

        let color = path_modul
            .state
            .print_type
            .as_ref()
            .unwrap_or(&crate::slicer::print_type::PrintType::Unknown)
            .get_color();

        for element in path_modul.paths.iter().enumerate() {
            let path = element.1;

            if element.0 == path_modul.paths.len() - 1 {
                if let Some(last) = last_cross.take() {
                    self.draw_rect_with_cross(&path.end, &last, &color);

                    self.finished_child(path_modul.state.clone(), path_modul.line_range);
                }
            }

            if path.print {
                let direction = path.direction();

                let cross = get_cross(direction, diameter / 2.0);

                if let Some(last) = last_cross.take() {
                    self.draw_cross_connection(&path.start, &cross, &last, &color);
                } else {
                    self.draw_rect_with_cross(&path.start, &cross, &color);
                }

                let flip = !adjust_faces(direction);

                self.draw_path((path.start, path.end), &color, !flip, &cross);
                last_cross = Some(cross);
            } else if let Some(last) = last_cross.take() {
                self.draw_rect_with_cross(&path.end, &last, &color);

                self.finished_child(path_modul.state.clone(), path_modul.line_range);
            }
        }
    }

    pub fn draw_path(
        &mut self,
        path: (Vector3<f32>, Vector3<f32>),
        color: &Srgba,
        flip: bool,
        cross: &Cross,
    ) {
        self.draw_rect_path(
            Rect3d {
                left_0: cross.up + path.0,
                left_1: cross.right + path.0,
                right_0: cross.up + path.1,
                right_1: cross.right + path.1,
            },
            color,
            flip,
            PathOrientation::SouthWest,
        );

        self.draw_rect_path(
            Rect3d {
                left_0: cross.down + path.0,
                left_1: cross.right + path.0,
                right_0: cross.down + path.1,
                right_1: cross.right + path.1,
            },
            color,
            flip,
            PathOrientation::NorthWest,
        );

        self.draw_rect_path(
            Rect3d {
                left_0: cross.down + path.0,
                left_1: cross.left + path.0,
                right_0: cross.down + path.1,
                right_1: cross.left + path.1,
            },
            color,
            flip,
            PathOrientation::NorthEast,
        );

        self.draw_rect_path(
            Rect3d {
                left_0: cross.up + path.0,
                left_1: cross.left + path.0,
                right_0: cross.up + path.1,
                right_1: cross.left + path.1,
            },
            color,
            flip,
            PathOrientation::SouthEast,
        );
    }

    pub fn draw_cross_connection(
        &mut self,
        center: &Vector3<f32>,
        start_cross: &Cross,
        end_cross: &Cross,
        color: &Srgba,
    ) {
        self.add_triangle(
            (
                end_cross.up + *center,
                end_cross.right + *center,
                start_cross.right + *center,
            ),
            color,
        );

        self.add_triangle(
            (
                end_cross.up + *center,
                end_cross.left + *center,
                start_cross.left + *center,
            ),
            color,
        );

        self.add_triangle(
            (
                end_cross.down + *center,
                end_cross.right + *center,
                start_cross.right + *center,
            ),
            color,
        );

        self.add_triangle(
            (
                end_cross.down + *center,
                end_cross.left + *center,
                start_cross.left + *center,
            ),
            color,
        );
    }

    fn draw_rect_path(
        &mut self,
        rect: Rect3d,
        color: &Srgba,
        face_flip: bool,
        orienation: PathOrientation,
    ) {
        let (mut triangle1, mut triangle2) = match orienation {
            PathOrientation::SouthEast => {
                let triangle1 = (rect.right_0, rect.left_1, rect.left_0);
                let triangle2 = (rect.right_0, rect.right_1, rect.left_1);

                (triangle1, triangle2)
            }
            PathOrientation::SouthWest => {
                let triangle1 = (rect.left_0, rect.left_1, rect.right_1);
                let triangle2 = (rect.right_1, rect.right_0, rect.left_0);

                (triangle1, triangle2)
            }
            PathOrientation::NorthEast => {
                let triangle1 = (rect.left_0, rect.left_1, rect.right_0);
                let triangle2 = (rect.left_1, rect.right_1, rect.right_0);

                (triangle1, triangle2)
            }
            PathOrientation::NorthWest => {
                let triangle1 = (rect.left_0, rect.right_0, rect.right_1);
                let triangle2 = (rect.right_1, rect.left_1, rect.left_0);

                (triangle1, triangle2)
            }
        };

        if face_flip {
            triangle1.reverse();
            triangle2.reverse();
        }

        self.add_triangle(triangle1, color);
        self.add_triangle(triangle2, color);
    }

    pub fn draw_rect(
        &mut self,
        point_left_0: Vector3<f32>,
        point_left_1: Vector3<f32>,
        point_right_0: Vector3<f32>,
        point_right_1: Vector3<f32>,
        color: &Srgba,
    ) {
        self.add_triangle((point_left_0, point_left_1, point_right_0), color);

        self.add_triangle((point_left_1, point_right_1, point_right_0), color);
    }

    pub fn draw_rect_with_cross(&mut self, center: &Vector3<f32>, cross: &Cross, color: &Srgba) {
        self.draw_rect(
            cross.up + *center,
            cross.right + *center,
            cross.down + *center,
            cross.left + *center,
            color,
        );
    }
}

pub struct Layers<'a>(pub &'a HashMap<usize, Layer>);

impl<'a> From<Layers<'a>> for three_d::CpuMesh {
    fn from(layers: Layers) -> Self {
        let mut positions = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();

        for entry in layers.0.iter() {
            let layer_mesh = entry.1;

            for position in layer_mesh.cpu_mesh.positions.iter() {
                positions.push(*position);
            }

            colors.reserve_exact(layer_mesh.cpu_mesh.colors.len());

            for color in layer_mesh.cpu_mesh.colors.iter() {
                colors.push(color.into());
                colors.push(color.into());
                colors.push(color.into());
            }
        }

        let mut cpu_mesh = three_d::CpuMesh {
            positions: Positions::F32(positions),
            colors: Some(colors),
            ..Default::default()
        };

        cpu_mesh.compute_normals();

        cpu_mesh
    }
}
