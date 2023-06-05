/*
	Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
	All rights reserved.
	Note: The complete copyright description for this software thesis can be found at the beginning of each file.
	Please refer to the terms and conditions stated therein.
*/

use bevy::prelude::{Component, Bundle};
use bevy::transform::components::Transform;

#[derive(PartialEq)]
pub enum Orbit {
    PrintBed,
    _WorkPiece
}

#[derive(Bundle)]
pub struct PossibleOrbitBundle {
    pub target: PossibleOrbitTarget,
    pub transform: Transform
}

#[derive(Component)]
pub struct PossibleOrbitTarget {
    _enabled: bool,
    pub orbit_type: Orbit,
}

impl PossibleOrbitTarget {

    pub fn new(orbit_type: Orbit) -> Self {
        Self { _enabled: orbit_type == Orbit::PrintBed, orbit_type }
    }

    pub fn _is_enabled(&self) -> bool {
        self._enabled
    }

    pub fn _enable(&mut self) {
        self._enabled = true;
    }

    pub fn _disable(&mut self) {
        self._enabled = false;
    }

}



