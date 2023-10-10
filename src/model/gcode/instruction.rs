use strum_macros::Display;

use super::{movement::Movements, state::State, SourceBuilder};

#[derive(Debug, Clone, Display, PartialEq)]
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Instruction {
    instruction_type: InstructionType,
    child_instructions: Vec<InstructionType>,
    movements: Movements,
}

#[allow(dead_code)]
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

    pub fn to_gcode(&self) -> String {
        let mut builder = SourceBuilder::new();

        builder.push_instruction(self.instruction_type.clone());
        builder.push_movements(&self.movements);

        for instruction in self.child_instructions.iter() {
            builder.push_instruction(instruction.clone())
        }

        builder.finish()
    }
}

#[derive(Debug, Clone)]
pub struct InstructionModul {
    instructions: Vec<Instruction>,
    start: usize,
    end: Option<usize>,
    state: State,
}

impl InstructionModul {
    pub fn empty() -> Self {
        Self {
            instructions: Vec::new(),
            start: 0,
            end: None,
            state: State::empty(),
        }
    }

    pub fn new(start: usize, state: State) -> Self {
        Self {
            instructions: Vec::new(),
            start,
            end: None,
            state,
        }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn finish(&mut self, end: usize) {
        self.end = Some(end);
    }

    pub fn range(&self) -> (usize, usize) {
        (self.start, self.end.unwrap_or(self.start))
    }
}
