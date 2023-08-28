use crate::slicer::print_type::PrintType;

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
#[allow(non_snake_case, dead_code)]
pub enum StateField {
    LAYER(usize),
    TYPE(PrintType),
    MESH(String),
}

impl TryFrom<String> for StateField {
    type Error = crate::error::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let variant = match s.split(':').next() {
            Some(variant) => variant,
            None => {
                return Err(crate::error::Error::GCodeStateParseError(
                    "Invalid GCode".into(),
                ))
            }
        };

        let value = match s.split(':').nth(1) {
            Some(variant) => variant,
            None => {
                return Err(crate::error::Error::GCodeStateParseError(
                    "Invalid State Change".into(),
                ))
            }
        };

        match variant {
            "LAYER" => {
                let value = value.parse::<usize>().map_err(|_| {
                    crate::error::Error::GCodeStateParseError("Invalid Layer".into())
                })?;

                Ok(StateField::LAYER(value))
            }
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
                    crate::error::Error::GCodeStateParseError("Invalid Print Type".into())
                })?;

                Ok(StateField::TYPE(value))
            }
            "MESH" => Ok(StateField::MESH(value.to_string())),
            _ => Err(crate::error::Error::GCodeStateParseError(
                "Invalid GCodeState Type".into(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub layer: Option<usize>,
    pub print_type: Option<PrintType>,
    pub mesh: Option<String>,
}

impl State {
    pub fn empty() -> Self {
        Self {
            layer: None,
            print_type: None,
            mesh: None,
        }
    }

    pub fn parse(&mut self, line: String) -> Result<(), crate::error::Error> {
        let variant: StateField = line.try_into()?;

        println!("State Change: {:?}", variant);

        match variant {
            StateField::LAYER(value) => {
                self.layer = Some(value);
            }
            StateField::TYPE(value) => {
                self.print_type = Some(value);
            }
            StateField::MESH(value) => {
                self.mesh = Some(value);
            }
        };

        Ok(())
    }
}
