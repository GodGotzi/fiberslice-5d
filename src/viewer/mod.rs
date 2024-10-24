use std::collections::BTreeSet;

use egui_code_editor::Syntax;
use parking_lot::RwLock;

use crate::{
    prelude::{Mode, WgpuContext},
    render::Vertex,
};

mod camera;
pub use camera::*;

pub mod select;
pub mod server;
pub mod toolpath;
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

pub trait Server {
    fn instance(context: &WgpuContext) -> Self;
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
    fn mode_changed(&mut self, mode: Mode) {}
}

#[derive(Debug)]
pub struct Viewer {
    pub env_server: RwLock<server::EnvironmentServer>,
    pub toolpath_server: RwLock<server::ToolpathServer>,
    pub model_server: RwLock<server::CADModelServer>,
}

impl Viewer {
    pub fn instance(context: &WgpuContext) -> Self {
        Self {
            env_server: RwLock::new(server::EnvironmentServer::instance(context)),
            toolpath_server: RwLock::new(server::ToolpathServer::instance(context)),
            model_server: RwLock::new(server::CADModelServer::instance(context)),
        }
    }

    pub fn mode_changed(&self, mode: Mode) {
        self.env_server.write().mode_changed(mode);
        self.toolpath_server.write().mode_changed(mode);
        self.model_server.write().mode_changed(mode);
    }
}

unsafe impl Send for Viewer {}
unsafe impl Sync for Viewer {}
