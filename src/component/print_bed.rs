
use bevy::prelude::*;

use crate::view::orbit::PossibleOrbitTarget;

#[derive(Component, Default)]
pub struct PrintBed;

#[derive(Bundle)]
pub struct PrintBedBundle {
    pub bed: PrintBed,
    pub orbit_target: PossibleOrbitTarget,
    #[bundle]
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}