use std::collections::BTreeSet;

use egui_code_editor::Syntax;
use parking_lot::RwLock;
use rether::vertex::Vertex;

pub mod gcode;
pub mod select;
pub mod server;
pub mod tracker;
pub mod volume;

pub struct Visual<const T: usize, const W: usize> {
    pub vertices: [Vertex; T],
    pub wires: [Vertex; W],
}

pub trait GCodeSyntax {
    fn gcode() -> Syntax;
}

impl GCodeSyntax for Syntax {
    fn gcode() -> Syntax {
        Syntax {
            language: "GCode",
            case_sensitive: true,
            comment: ";",
            comment_multiline: [r#";;"#, r#";;"#],
            hyperlinks: BTreeSet::from([]),
            keywords: BTreeSet::from([
                "G0", "G1", "G2", "G3", "G4", "G10", "G17", "G18", "G19", "G20", "G21", "G28",
            ]),
            types: BTreeSet::from(["X", "Y", "Z", "E", "F"]),
            special: BTreeSet::from(["False", "None", "True"]),
        }
    }
}

#[derive(Debug)]
pub struct Viewer {
    pub env_server: RwLock<server::EnvironmentServer>,
    pub toolpath_server: RwLock<server::ToolpathServer>,
    pub model_server: RwLock<server::CADModelServer>,
    select: RwLock<select::Selector>,
}

impl Viewer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self {
            env_server: RwLock::new(server::EnvironmentServer::new(device, queue)),
            toolpath_server: RwLock::new(server::ToolpathServer::new(device)),
            model_server: RwLock::new(server::CADModelServer::new(device)),
            select: RwLock::new(select::Selector::default()),
        }
    }

    pub fn selector(&self) -> &RwLock<select::Selector> {
        &self.select
    }
}

unsafe impl Send for Viewer {}
unsafe impl Sync for Viewer {}
