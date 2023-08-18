use self::{instruction::InstructionType, movement::Movements};

pub mod instruction;
pub mod movement;
pub mod parser;

pub struct GCodeSourceBuilder {
    first: bool,
    source: String,
}

impl GCodeSourceBuilder {
    pub fn new() -> Self {
        Self {
            first: true,
            source: String::new(),
        }
    }

    pub fn push_movements(&mut self, movements: &Movements) {
        if let Some(x) = movements.X.as_ref() {
            self.push_movement("X", *x);
        }

        if let Some(y) = movements.Y.as_ref() {
            self.push_movement("Y", *y);
        }

        if let Some(z) = movements.Z.as_ref() {
            self.push_movement("Z", *z);
        }

        if let Some(e) = movements.E.as_ref() {
            self.push_movement("E", *e);
        }

        if let Some(f) = movements.F.as_ref() {
            self.push_movement("F", *f);
        }
    }

    pub fn push_movement(&mut self, movement_str: &str, value: f64) {
        if !self.first {
            self.source.push(' ');
        } else {
            self.first = false;
        }

        let code = format!("{}{}", movement_str, value);

        self.source.push_str(code.as_str());
    }

    pub fn push_instruction(&mut self, instruction: InstructionType) {
        if !self.first {
            self.source.push(' ');
        } else {
            self.first = false;
        }

        self.source.push_str(instruction.to_string().as_str());
    }

    pub fn finish(self) -> String {
        self.source
    }
}

#[derive(Debug, Clone)]
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
