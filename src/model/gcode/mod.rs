pub mod instruction;
pub mod movement;
pub mod parser;

pub struct GCode {
    instructions: Vec<instruction::Instruction>,
}

impl GCode {
    pub fn new(instructions: Vec<instruction::Instruction>) -> GCode {
        GCode { instructions }
    }

    pub fn instructions(&self) -> &Vec<instruction::Instruction> {
        &self.instructions
    }
}
