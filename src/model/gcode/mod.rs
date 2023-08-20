use type_eq_derive::TypeHolder;

use crate::slicer::print_type::PrintType;

use self::{
    instruction::{InstructionModul, InstructionType},
    movement::Movements,
};

pub mod instruction;
pub mod movement;
pub mod parser;
pub mod toolpath;

/*
;FLAVOR:Marlin
;TIME:2588
;Filament used: 1.50857m
;Layer height: 0.16
;MINX:73.343
;MINY:110.741
;MINZ:0.3
;MAXX:176.658
;MAXY:141.507
;MAXZ:10.06
;Generated with Cura_SteamEngine 5.2.1
M140 S60
M105
M190 S60
M104 S195
M105
M109 S195
M82 ;absolute extrusion mode
G21 ;metric values
G90 ;absolute positioning
M82 ;set extruder to absolute mode
M107 ;start with the fan off
G28 X0 Y0 ;move X/Y to min endstops
M300 S1318 P266
G28 Z0 ;move Z to min endstops
G0 Z0.2
G92 E0 ;zero the extruded length
G1 X40 E25 F400 ; Extrude 25mm of filament in a 4cm line. Reduce speed (F) if you have a nozzle smaller than 0.4mm!
G92 E0 ;zero the extruded length again
G1 E-1 F500 ; Retract a little
G1 X80 F4000 ; Quickly wipe away from the filament line
M117 ; Printing…
G5
G92 E0
G92 E0
G1 F1500 E-6.5
;LAYER_COUNT:62
;LAYER:0
M107
G0 F3000 X75.197 Y111.259 Z0.3
;TYPE:SKIRT
*/

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PermanentGCodeState {
    flavor: String,
    time: usize,
    filament_used: f64,
    layer_height: f64,
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

#[derive(Debug, Clone)]
pub struct GCodeState {
    pub states: Vec<GCodeStates>,
}

impl GCodeState {
    pub fn empty() -> Self {
        Self { states: Vec::new() }
    }
}

#[derive(Debug, Clone, TypeHolder)]
#[allow(clippy::upper_case_acronyms)]
#[allow(non_snake_case, dead_code)]
pub enum GCodeStates {
    LAYER(usize),
    TYPE(PrintType),
    MESH(String),
}

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
    instruction_moduls: Vec<InstructionModul>,
}

impl GCode {
    pub fn new(moduls: Vec<InstructionModul>) -> Self {
        Self {
            instruction_moduls: moduls,
        }
    }

    pub fn instructions(&self) -> &Vec<InstructionModul> {
        &self.instruction_moduls
    }
}
