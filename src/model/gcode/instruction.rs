use super::movement::Movements;

pub enum InstructionType {
    G1,
    G0,
    G24,
    M30,
    /*
    .
    .
    .
    .

    TODO
     */
}

impl InstructionType {
    pub fn is_stand_alone(&self) -> bool {
        match self {
            InstructionType::G1 => false,
            InstructionType::G0 => false,
            InstructionType::G24 => true,
            InstructionType::M30 => true,
        }
    }
}

impl TryFrom<&str> for InstructionType {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "G1" => Ok(InstructionType::G1),
            "G0" => Ok(InstructionType::G0),
            "G24" => Ok(InstructionType::G24),
            "M30" => Ok(InstructionType::M30),

            _ => Err(crate::error::Error::UnknownInstructionType(format!(
                "Unknown instruction type: {}",
                value
            ))),
        }
    }
}

pub struct Instruction {
    instruction_type: InstructionType,
    child_instructions: Vec<InstructionType>,
    movements: Movements,
}

impl Instruction {
    pub fn new(
        instruction_type: InstructionType,
        child_instructions: Vec<InstructionType>,
        movements: Movements,
    ) -> Instruction {
        Instruction {
            instruction_type,
            child_instructions,
            movements,
        }
    }

    pub fn instruction_type(&self) -> &InstructionType {
        &self.instruction_type
    }

    pub fn child_instructions(&self) -> &Vec<InstructionType> {
        &self.child_instructions
    }

    pub fn movements(&self) -> &Movements {
        &self.movements
    }
}
