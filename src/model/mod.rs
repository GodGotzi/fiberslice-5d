use bevy::prelude::Component;

pub mod gcode;
pub mod layer;

#[derive(Component)]
pub struct ToolPath;

pub mod tests {

    #[test]
    pub fn test_fliper() {
        use bevy::math::vec3;

        use crate::model::layer::PathOrientation;

        use super::layer::PathOrientationFlipper;

        let direction = vec3(0.0, -30.0, 0.0);
        let fliper = PathOrientationFlipper::from(&direction);

        assert_eq!(
            fliper.flip(PathOrientation::NorthEast),
            PathOrientation::SouthWest
        );

        assert_eq!(
            fliper.flip(PathOrientation::NorthWest),
            PathOrientation::SouthEast
        );

        assert_eq!(
            fliper.flip(PathOrientation::SouthEast),
            PathOrientation::NorthWest
        );

        assert_eq!(
            fliper.flip(PathOrientation::SouthWest),
            PathOrientation::NorthEast
        );
    }
}
