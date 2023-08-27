#[test]
fn test_gcode_builder() {
    use crate::model::gcode::{
        self,
        toolpath::{self, PathModulMesh},
    };
    use crate::model::gcode::{instruction::InstructionModul, state::State};
    use crate::model::gcode::{
        instruction::{Instruction, InstructionType},
        movement::Movements,
    };

    let mut moduls = Vec::new();

    let state = State {
        layer: Some(1),
        print_type: Some(crate::slicer::print_type::PrintType::Perimeter),
        mesh: Some("mesh".to_string()),
    };

    let mut modul = InstructionModul::new(state);

    let mut movements = Movements::new();
    movements.set_movement("X", 120.0);
    movements.set_movement("E", 0.5);

    modul.push(Instruction::new(InstructionType::G1, Vec::new(), movements));

    let mut movements = Movements::new();
    movements.set_movement("Y", 90.0);

    modul.push(Instruction::new(InstructionType::G1, Vec::new(), movements));

    moduls.push(modul);

    let gcode = gcode::GCode::new(moduls);

    let toolpath = toolpath::ToolPath::from(gcode);

    let meshes: Vec<PathModulMesh> = std::convert::Into::<Vec<PathModulMesh>>::into(toolpath);

    println!("{:?}", meshes);

    for mesh in meshes {
        println!("{:?}", mesh.mesh().positions.to_f64());
    }
}
