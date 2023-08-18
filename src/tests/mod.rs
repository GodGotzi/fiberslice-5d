#[test]
fn test_gcode_builder() {
    use crate::model::gcode::{
        instruction::{Instruction, InstructionType},
        movement::Movements,
    };

    let mut movements = Movements::new();
    movements.set_movement("X", 120.0);
    movements.set_movement("Y", 200.0);

    let instruction = Instruction::new(InstructionType::G0, Vec::new(), movements);

    assert_eq!(instruction.to_gcode(), "G0 X120 Y200");
}
