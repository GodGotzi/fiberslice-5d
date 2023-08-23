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
