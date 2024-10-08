use crate::slicer::{PathType, PrintType};

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case, dead_code)]
enum StateField {
    LAYER_CHANGE,
    LAYER(usize),
    TYPE(PrintType),
    MESH(String),
}

impl TryFrom<String> for StateField {
    type Error = crate::error::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let splited = s.split(':').collect::<Vec<&str>>();

        let variant = match splited.first() {
            Some(variant) => *variant,
            None => return Err(crate::error::Error::GCodeStateParse("Invalid GCode".into())),
        };

        let value = match splited.get(1) {
            Some(variant) => *variant,
            None => "",
        };

        match variant {
            "LAYER" => {
                let value = value
                    .parse::<usize>()
                    .map_err(|_| crate::error::Error::GCodeStateParse("Invalid Layer".into()))?;

                Ok(StateField::LAYER(value))
            }
            "LAYER_CHANGE" => Ok(StateField::LAYER_CHANGE),
            "TYPE" => {
                let mut value = value.trim().to_ascii_lowercase();
                value[0..1].make_ascii_uppercase();

                while let Some(index) = value.find(' ') {
                    if value.len() > index + 2 {
                        value[index..index + 2].make_ascii_uppercase();
                    }

                    value.replace_range(index..index + 1, "");
                }

                while let Some(index) = value.find('-') {
                    if value.len() > index + 2 {
                        value[index..index + 2].make_ascii_uppercase();
                    }

                    value.replace_range(index..index + 1, "");
                }

                let value = value.parse::<PrintType>().map_err(|_| {
                    crate::error::Error::GCodeStateParse("Invalid Print Type".into())
                })?;

                Ok(StateField::TYPE(value))
            }
            "MESH" => Ok(StateField::MESH(value.to_string())),
            _ => Err(crate::error::Error::GCodeStateParse(
                "Invalid GCodeState Type".into(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintState {
    pub path_type: PathType,
    pub layer: Option<usize>,
    pub mesh: Option<String>,
}

impl PrintState {
    pub fn empty() -> Self {
        Self {
            layer: None,
            path_type: PathType::Setup,
            mesh: None,
        }
    }

    pub fn parse(&mut self, line: String) -> Result<(), crate::error::Error> {
        let variant: StateField = line.try_into()?;

        match variant {
            StateField::LAYER(value) => {
                self.layer = Some(value);
            }
            StateField::LAYER_CHANGE => {
                self.layer = Some(self.layer.unwrap_or(0) + 1);
            }
            StateField::TYPE(value) => {
                self.path_type.update_type(value);
            }
            StateField::MESH(value) => {
                self.mesh = Some(value);
            }
        };

        Ok(())
    }
}
