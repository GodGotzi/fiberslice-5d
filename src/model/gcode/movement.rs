#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Movements {
    pub X: Option<f64>,
    pub Y: Option<f64>,
    pub Z: Option<f64>,
    pub E: Option<f64>,
    pub F: Option<f64>,
}

impl Movements {
    pub fn new() -> Movements {
        Movements {
            X: None,
            Y: None,
            Z: None,
            E: None,
            F: None,
        }
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
}
