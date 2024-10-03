use std::fmt::Debug;

use crate::{prelude::WgpuContext, viewer::volume::Volume};

#[derive(Debug)]
pub struct EnvironmentServer {
    volume: Volume,
}

impl EnvironmentServer {
    pub fn instance(_context: &WgpuContext) -> Self {
        Self {
            volume: Volume::instance(),
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.volume.render(render_pass);
    }

    pub fn render_lines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.volume.render_lines(render_pass);
    }
}
