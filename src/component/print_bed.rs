/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/


use bevy::prelude::*;

use crate::view::orbit::PossibleOrbitTarget;

#[derive(Component, Default)]
pub struct PrintBed;

#[derive(Bundle)]
pub struct PrintBedBundle {
    pub bed: PrintBed,
    pub orbit_target: PossibleOrbitTarget,
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}