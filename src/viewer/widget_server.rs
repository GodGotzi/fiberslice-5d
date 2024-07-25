use std::collections::HashMap;

use crate::{
    model::TreeHandle,
    picking::hitbox::HitboxNode,
    render::{buffer::DynamicBuffer, vertex::Vertex},
};

use super::Visual;

mod layout {

    mod wire {
        use crate::render::{
            buffer::alloc::{BufferAlloc, BufferAllocation},
            vertex::Vertex,
        };

        const HOVER_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
            offset: 0,
            size: 48,
        };

        const SELECT_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
            offset: HOVER_BOX_ALLOCATION.size,
            size: 48,
        };

        #[derive(Debug)]
        pub struct WireAllocator;

        impl BufferAlloc<Vertex> for WireAllocator {
            fn get(&self, id: &str) -> Option<&BufferAllocation> {
                match id {
                    "hover_box" => Some(&HOVER_BOX_ALLOCATION),
                    "select_box" => Some(&SELECT_BOX_ALLOCATION),
                    _ => None,
                }
            }

            fn size(&self) -> usize {
                HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size
            }
        }
    }

    const HOVER_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
        offset: 0,
        size: 72,
    };

    const SELECT_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
        offset: HOVER_BOX_ALLOCATION.size,
        size: 72,
    };

    #[derive(Debug)]
    pub struct WidgetAllocator;
    pub use wire::WireAllocator;

    impl BufferAlloc<Vertex> for WidgetAllocator {
        fn get(&self, id: &str) -> Option<&BufferAllocation> {
            match id {
                "hover_box" => Some(&HOVER_BOX_ALLOCATION),
                "select_box" => Some(&SELECT_BOX_ALLOCATION),
                _ => None,
            }
        }

        fn size(&self) -> usize {
            HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size
        }
    }

    use crate::render::{
        buffer::alloc::{BufferAlloc, BufferAllocation},
        vertex::Vertex,
    };
}

#[derive(Debug)]
struct WidgetContext;

#[derive(Debug)]
pub struct WidgetHandle {
    handle: TreeHandle<WidgetContext>,
}

#[derive(Debug)]
pub struct WidgetServer {
    widget_hitbox: HitboxNode,
    buffer: DynamicBuffer<Vertex, layout::WidgetAllocator>,
    line_buffer: DynamicBuffer<Vertex, layout::WireAllocator>,

    widgets: HashMap<String, WidgetHandle>,
}

impl WidgetServer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            widget_hitbox: HitboxNode::root(),
            buffer: DynamicBuffer::new(layout::WidgetAllocator, "Widget Buffer", device),
            line_buffer: DynamicBuffer::new(layout::WireAllocator, "Widget Line Buffer", device),
            widgets: HashMap::new(),
        }
    }

    pub fn set_hover_visual(&mut self, visual: Visual<72, 48>, queue: &wgpu::Queue) {
        self.buffer.write(queue, "select_box", &visual.vertices);
        self.line_buffer.write(queue, "select_box", &visual.wires);
    }

    pub fn set_select_visual(&mut self, visual: Visual<72, 48>, queue: &wgpu::Queue) {
        self.buffer.write(queue, "hover_box", &visual.vertices);
        self.line_buffer.write(queue, "hover_box", &visual.wires);
    }

    pub fn reset_hover_visual(&mut self, queue: &wgpu::Queue) {
        self.buffer.write(queue, "hover_box", &[]);
        self.line_buffer.write(queue, "hover_box", &[]);
    }

    pub fn reset_select_visual(&mut self, queue: &wgpu::Queue) {
        self.buffer.write(queue, "select_box", &[]);
        self.line_buffer.write(queue, "select_box", &[]);
    }

    pub fn read_buffer(&self) -> &DynamicBuffer<Vertex, layout::WidgetAllocator> {
        &self.buffer
    }

    pub fn read_line_buffer(&self) -> &DynamicBuffer<Vertex, layout::WireAllocator> {
        &self.line_buffer
    }
}
