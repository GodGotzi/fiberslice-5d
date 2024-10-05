use std::str::SplitWhitespace;

use super::{
    instruction::{Instruction, InstructionModul, InstructionType},
    movement::Movements,
    state::PrintState,
    GCode,
};

pub fn parse_content(content: &str) -> Result<GCode, crate::error::Error> {
    let mut lines: Vec<&str> = content.lines().collect();

    let mut moduls: Vec<InstructionModul> = Vec::new();

    let mut modul: Option<InstructionModul> = Some(InstructionModul::empty());

    for (index, line) in lines.iter_mut().enumerate() {
        *line = line.trim();
        let current_state = modul.as_ref().unwrap().state.clone();

        if line.starts_with(';') {
            compute_comment_with_prefix(line, index, &mut modul, &mut moduls);
        } else if let Some((instruction, comment)) = line.split_once(';') {
            if let Some(instruction_result) =
                compute_instruction(instruction, index, current_state)?
            {
                match instruction_result {
                    InstructionResult::Instruction(instruction) => {
                        modul.as_mut().unwrap().push(instruction);
                    }
                    InstructionResult::NewInstructionModul(new_modul) => {
                        moduls.push(modul.take().unwrap());
                        modul = Some(new_modul);
                    }
                }
            }

            if !comment.is_empty() {
                compute_comment(comment, index, &mut modul, &mut moduls);
            }
        } else if let Some(instruction_result) = compute_instruction(line, index, current_state)? {
            match instruction_result {
                InstructionResult::Instruction(instruction) => {
                    modul.as_mut().unwrap().push(instruction);
                }
                InstructionResult::NewInstructionModul(new_modul) => {
                    moduls.push(modul.take().unwrap());
                    modul = Some(new_modul);
                }
            }
        }
    }

    if !modul.as_ref().unwrap().is_empty() {
        moduls.push(modul.take().unwrap());
    }

    Ok(moduls)
}

fn compute_comment_with_prefix(
    line: &str,
    count: usize,
    modul: &mut Option<InstructionModul>,
    moduls: &mut Vec<InstructionModul>,
) {
    compute_comment(line.strip_prefix(';').unwrap(), count, modul, moduls);
}

fn compute_comment(
    line: &str,
    count: usize,
    modul: &mut Option<InstructionModul>,
    moduls: &mut Vec<InstructionModul>,
) {
    if modul.as_ref().unwrap().is_empty() {
        let _ = parse_comment_into_state(line, &mut modul.as_mut().unwrap().state);
    } else if let Ok(result) = parse_comment_to_state(line, modul.as_ref().unwrap().state.clone()) {
        let mut owned_modul = modul.take().unwrap();
        owned_modul.finish(count - 1);

        moduls.push(owned_modul);

        *modul = Some(InstructionModul::new(count, result));
    }
}

enum InstructionResult {
    Instruction(Instruction),
    NewInstructionModul(InstructionModul),
}

fn compute_instruction(
    line: &str,
    index: usize,
    mut current_state: PrintState,
) -> Result<Option<InstructionResult>, crate::prelude::Error> {
    let mut parameters = line.split_whitespace();
    if let Some(next) = parameters.next() {
        if let Ok(main_instruction) = next.try_into() {
            let mut child_instructions: Vec<InstructionType> = Vec::new();

            let mut movements = Movements::new();

            compute_parameters(parameters, &mut child_instructions, &mut movements, index)?;

            if (main_instruction == InstructionType::G0 || movements.E.unwrap_or(0.0) <= 0.0)
                && !current_state.path_type.is_travel()
            {
                current_state.path_type.set_travel(true);

                let mut new_modul = InstructionModul::new(index, current_state);

                new_modul.push(Instruction::new(
                    main_instruction,
                    child_instructions,
                    movements,
                ));

                return Ok(Some(InstructionResult::NewInstructionModul(new_modul)));
            } else if (main_instruction == InstructionType::G1 && movements.E.unwrap_or(0.0) > 0.0)
                && current_state.path_type.is_travel()
            {
                current_state.path_type.set_travel(false);

                let mut new_modul = InstructionModul::new(index, current_state);

                new_modul.push(Instruction::new(
                    main_instruction,
                    child_instructions,
                    movements,
                ));

                return Ok(Some(InstructionResult::NewInstructionModul(new_modul)));
            } else {
                return Ok(Some(InstructionResult::Instruction(Instruction::new(
                    main_instruction,
                    child_instructions,
                    movements,
                ))));
            }
        }
    };

    Ok(None)
}

fn parse_comment_to_state(
    line: &str,
    mut last_state: PrintState,
) -> Result<PrintState, crate::error::Error> {
    parse_comment_into_state(line, &mut last_state)?;

    Ok(last_state)
}

fn parse_comment_into_state(
    line: &str,
    last_state: &mut PrintState,
) -> Result<(), crate::error::Error> {
    last_state.parse(line.to_string())
}

fn compute_parameters(
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
                return Err(crate::error::Error::GCodeParse(
                    format!("Invalid instruction type: {}", movement),
                    index,
                ));
            }
        } else if word.starts_with('M') {
            println!("Ingoring M instruction: {}", word);
        } else if Movements::is_movement(movement) {
            movements.set_movement(
                movement,
                value.parse::<f32>().map_err(|_| {
                    crate::error::Error::GCodeParse(
                        format!("Invalid movement value: {}", value),
                        index,
                    )
                })?,
            );
        }
    }

    Ok(())
}
