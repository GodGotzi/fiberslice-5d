use three_d_asset::Vector3;

use super::SourceBuilder;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct Movements {
    pub X: Option<f64>,
    pub Y: Option<f64>,
    pub Z: Option<f64>,
    pub E: Option<f64>,
    pub F: Option<f64>,
}

impl Movements {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_movement(movement_str: &str) -> bool {
        matches!(movement_str, "X" | "Y" | "Z" | "E" | "F")
    }

    pub fn set_movement(&mut self, movement_str: &str, value: f64) {
        match movement_str {
            "X" => self.X = Some(value),
            "Y" => self.Y = Some(value),
            "Z" => self.Z = Some(value),
            "E" => self.E = Some(value),
            "F" => self.F = Some(value),
            _ => (),
        }
    }

    pub fn add_movements(&mut self, movements: &Movements) {
        if let Some(x) = movements.X.as_ref() {
            self.X = Some(*x);
        }

        if let Some(y) = movements.Y.as_ref() {
            self.Y = Some(*y);
        }

        if let Some(z) = movements.Z.as_ref() {
            self.Z = Some(*z);
        }

        if let Some(e) = movements.E.as_ref() {
            self.E = Some(*e);
        }

        if let Some(f) = movements.F.as_ref() {
            self.F = Some(*f);
        }
    }

    pub fn to_vec3(&self, zero: Vector3<f64>) -> Vector3<f64> {
        let mut vec = zero;

        if let Some(x) = self.X.as_ref() {
            vec.x = *x;
        }

        if let Some(y) = self.Y.as_ref() {
            vec.y = *y;
        }

        if let Some(z) = self.Z.as_ref() {
            vec.z = *z;
        }

        vec
    }

    pub fn to_gcode(&self) -> String {
        let mut builder = SourceBuilder::new();

        if let Some(x) = self.X.as_ref() {
            builder.push_movement("X", *x);
        }

        if let Some(y) = self.Y.as_ref() {
            builder.push_movement("Y", *y);
        }

        if let Some(z) = self.Z.as_ref() {
            builder.push_movement("Z", *z);
        }

        if let Some(e) = self.E.as_ref() {
            builder.push_movement("E", *e);
        }

        if let Some(f) = self.F.as_ref() {
            builder.push_movement("F", *f);
        }

        builder.finish()
    }
}
