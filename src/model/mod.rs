use three_d::{Gm, Mesh, PhysicalMaterial};

pub mod gcode;
pub mod mesh;
pub mod shapes;

pub struct Model {
    inner: Gm<Mesh, PhysicalMaterial>,
}

impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("A Model").finish()
    }
}
