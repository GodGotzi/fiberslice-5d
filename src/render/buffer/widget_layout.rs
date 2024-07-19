use crate::render::vertex::Vertex;

use super::layout::{BufferAlloc, BufferAllocation};

pub const HOVER_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
    offset: 0,
    size: 12,
};

pub const SELECT_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
    offset: 12,
    size: 12,
};

pub struct WidgetBufferLayout;

impl BufferAlloc<Vertex> for WidgetBufferLayout {
    fn write(
        &self,
        buffer: &mut super::DynamicBuffer<Vertex>,
        queue: &wgpu::Queue,
        id: &str,
        data: &[Vertex],
    ) {
    }

    fn read<'a>(
        &self,
        buffer: &'a mut super::DynamicBuffer<Vertex>,
        id: &str,
    ) -> Option<&'a [Vertex]> {
        todo!()
    }

    fn get(&self, id: &str) -> Option<&BufferAllocation> {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}
