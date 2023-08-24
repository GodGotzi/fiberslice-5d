use std::str::SplitWhitespace;

use super::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
    state::State,
    GCode,
};

pub fn parse_content(content: &str) -> Result<GCode, crate::error::Error> {
    let mut lines: Vec<&str> = content.lines().collect();

    let mut moduls: Vec<InstructionModul> = Vec::new();

    let mut modul: Option<InstructionModul> = Some(InstructionModul::empty());

    for (index, line) in lines.iter_mut().enumerate() {
        *line = line.trim();

        if line.starts_with(';') {
            if modul.as_ref().unwrap().is_empty() {
                let _ = parse_comment_into_state(line, modul.as_mut().unwrap().gcode_state_mut());
            } else if let Ok(result) =
                parse_comment_to_state(line, modul.as_ref().unwrap().gcode_state().clone())
            {
                moduls.push(modul.take().unwrap());

                modul = Some(InstructionModul::new(result));
            }
        }

        let mut parameters = line.split_whitespace();

        let main_instruction: InstructionType = parameters.next().unwrap().try_into()?;

        let mut child_instructions: Vec<InstructionType> = Vec::new();

        let mut movements = Movements::new();

        compute_parameters(parameters, &mut child_instructions, &mut movements, index)?;

        let instruction = crate::model::gcode::instruction::Instruction::new(
            main_instruction,
            child_instructions,
            movements,
        );

        modul.as_mut().unwrap().push(instruction);
    }

    let code = GCode::new(moduls);

    Ok(code)
}

pub fn parse_comment_to_state(
    line: &str,
    mut last_state: State,
) -> Result<State, crate::error::Error> {
    parse_comment_into_state(line, &mut last_state)?;

    Ok(last_state)
}

pub fn parse_comment_into_state(
    line: &str,
    last_state: &mut State,
) -> Result<(), crate::error::Error> {
    last_state.parse(line.to_string())
}

pub fn compute_parameters(
    parameters: SplitWhitespace<'_>,
    child_instructions: &mut Vec<InstructionType>,
    movements: &mut Movements,
    index: usize,
) -> Result<(), crate::error::Error> {
    for word in parameters {
        let (movement, value) = word.split_at(1);

        if word.starts_with('G') {
            let instruction_type = InstructionType::try_from(movement)?;

            if instruction_type.is_stand_alone() {
                child_instructions.push(instruction_type);
            } else {
                return Err(crate::error::Error::GCodeParseError(
                    format!("Invalid instruction type: {}", movement),
                    index,
                ));
            }
        } else if word.starts_with('M') {
            println!("Ingoring M instruction: {}", word);
        } else if Movements::is_movement(movement) {
            movements.set_movement(
                movement,
                value.parse::<f64>().map_err(|_| {
                    crate::error::Error::GCodeParseError(
                        format!("Invalid movement value: {}", value),
                        index,
                    )
                })?,
            );
        }
    }

    Ok(())
}

impl TryFrom<&str> for GCode {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let code = parse_content(value)?;

        Ok(code)
    }
}
