use three_d::{Gm, Mesh, PhysicalMaterial, WindowedContext};
use three_d_asset::{Positions, Srgba, TriMesh, Vector3};

use crate::utils::FlipYZ;

use super::gcode::state::State;

pub struct PartCoordinator {
    positions: Vec<Vector3<f64>>,
    colors: Vec<Srgba>,
}

impl PartCoordinator {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
        }
    }

    pub fn add_position(&mut self, position: Vector3<f64>) {
        self.positions.push(position.flip_yz());
    }

    #[allow(dead_code)]
    pub fn add_color(&mut self, color: Srgba) {
        self.colors.push(color);
    }

    pub fn add_color_3_times(&mut self, color: Srgba) {
        self.colors.push(color);
        self.colors.push(color);
        self.colors.push(color);
    }

    pub fn next_trimesh(&mut self) -> TriMesh {
        let positions = Positions::F64(self.positions.clone());
        let colors = Some(self.colors.clone());

        self.positions.clear();
        self.colors.clear();

        let mut mesh = TriMesh {
            positions,
            colors,
            ..Default::default()
        };

        mesh.compute_normals();
        mesh
    }
}

pub trait MeshGroup<G> {
    fn no_parent(mesh: TriMesh) -> Self;
    fn parent(meshes: Vec<LayerPartMesh>) -> Self;
    fn into_model(self, context: &WindowedContext) -> G;
    fn tri_count(&self) -> usize;
}

#[derive(Debug)]
pub struct LayerMesh {
    main: TriMesh,
    child_meshes: Vec<LayerPartMesh>,
}

impl LayerMesh {
    pub fn new(main: TriMesh, child_meshes: Vec<LayerPartMesh>) -> Self {
        Self { main, child_meshes }
    }
}

impl From<Vec<LayerPartMesh>> for LayerMesh {
    fn from(child_meshes: Vec<LayerPartMesh>) -> Self {
        let mut positions = Vec::new();
        let mut colors = Vec::new();

        for mesh in child_meshes.iter() {
            positions.extend(mesh.main.positions.to_f64().iter());
            colors.extend(mesh.main.colors.as_ref().unwrap().iter());
        }

        let mut mesh = TriMesh {
            positions: Positions::F64(positions),
            colors: Some(colors),
            ..Default::default()
        };

        mesh.compute_normals();

        Self {
            main: mesh,
            child_meshes,
        }
    }
}

impl MeshGroup<LayerModel> for LayerMesh {
    fn no_parent(mesh: TriMesh) -> Self {
        Self {
            main: mesh,
            child_meshes: Vec::new(),
        }
    }

    fn parent(meshes: Vec<LayerPartMesh>) -> Self {
        let mut positions = Vec::new();
        let mut colors = Vec::new();

        for mesh in meshes.iter() {
            positions.extend(mesh.main.positions.to_f64().iter());
            colors.extend(mesh.main.colors.as_ref().unwrap().iter());
        }

        let mesh = TriMesh {
            positions: Positions::F64(positions),
            colors: Some(colors),
            ..Default::default()
        };

        Self {
            main: mesh,
            child_meshes: meshes,
        }
    }

    fn into_model(self, context: &WindowedContext) -> LayerModel {
        let model = Gm::new(
            Mesh::new(context, &self.main),
            PhysicalMaterial {
                name: "FilamentMat".into(),
                ..Default::default()
            },
        );

        let mut min_line = usize::MAX;
        let mut max_line = usize::MIN;

        let child_models = self
            .child_meshes
            .into_iter()
            .map(|mesh| {
                min_line = std::cmp::min(min_line, mesh.line_range.0);
                max_line = std::cmp::max(max_line, mesh.line_range.1);
                mesh.into_model(context)
            })
            .collect();

        LayerModel {
            model,
            line_range: (0, 0),
            child_models,
        }
    }

    fn tri_count(&self) -> usize {
        self.main.positions.len() / 3
    }
}

pub struct LayerModel {
    pub model: Gm<Mesh, PhysicalMaterial>,
    pub line_range: (usize, usize),
    pub child_models: Vec<LayerPartModel>,
}

pub struct LayerPartModel {
    pub model: Gm<Mesh, PhysicalMaterial>,
    pub state: State,
    pub line_range: (usize, usize),
    pub child_models: Vec<Mesh>,
}

#[derive(Debug)]
pub struct LayerPartMesh {
    main: TriMesh,
    state: State,
    line_range: (usize, usize),
    child_meshes: Vec<TriMesh>,
}

impl LayerPartMesh {
    pub fn new(
        main: TriMesh,
        state: State,
        line_range: (usize, usize),
        child_meshes: Vec<TriMesh>,
    ) -> Self {
        Self {
            main,
            state,
            line_range,
            child_meshes,
        }
    }
}

impl LayerPartMesh {
    fn into_model(self, context: &WindowedContext) -> LayerPartModel {
        let model = Gm::new(
            Mesh::new(context, &self.main),
            PhysicalMaterial {
                name: "FilamentMat".into(),
                ..Default::default()
            },
        );

        LayerPartModel {
            model,
            state: self.state,
            line_range: self.line_range,
            child_models: self
                .child_meshes
                .into_iter()
                .map(|mesh| Mesh::new(context, &mesh))
                .collect(),
        }
    }
}
