use bevy::prelude::{Component, Bundle};
use bevy::transform::components::Transform;

#[derive(PartialEq)]
pub enum Orbit {
    PrintBed,
    WorkPiece
}

#[derive(Bundle)]
pub struct PossibleOrbitBundle {
    pub target: PossibleOrbitTarget,
    pub transform: Transform
}

#[derive(Component)]
pub struct PossibleOrbitTarget {
    enabled: bool,
    pub orbit_type: Orbit,
}

impl PossibleOrbitTarget {

    pub fn new(orbit_type: Orbit) -> Self {
        Self { enabled: orbit_type == Orbit::PrintBed, orbit_type }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

}



