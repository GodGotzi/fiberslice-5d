use std::collections::HashMap;

use rether::{
    alloc::{ModifyAction, StaticAllocHandle},
    model::{BaseModel, TreeModel},
    picking::{interact::Interactive, HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer,
};

use super::Visual;

mod layout {

    mod wire {
        use rether::{
            alloc::{BufferAlloc, BufferAllocation},
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

        #[derive(Debug, Default)]
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

    #[derive(Debug, Default)]
    pub struct WidgetAllocator;
    use rether::{
        alloc::{BufferAlloc, BufferAllocation, StaticAllocHandle},
        vertex::Vertex,
    };
    pub use wire::WireAllocator;

    impl BufferAlloc<Vertex> for WidgetAllocator {
        type Handle = StaticAllocHandle<Vertex>;

        fn get(&self, id: &str) -> Option<&std::sync::Arc<Self::Handle>> {
            match id {
                "hover_box" => Some(&HOVER_BOX_ALLOCATION),
                "select_box" => Some(&SELECT_BOX_ALLOCATION),
                _ => None,
            }
        }

        fn size(&self) -> usize {
            HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size
        }

        fn update(&self, modify: impl Fn(rether::alloc::ModifyAction<Vertex>)) {}
    }
}

#[derive(Debug)]
struct WidgetContext;

#[derive(Debug)]
pub struct WidgetModel {
    handle: TreeModel<Vertex, WidgetContext, StaticAllocHandle<Vertex>>,
}

#[derive(Debug)]
pub struct WidgetServer {
    widget_hitbox: HitboxRoot<BaseModel<Vertex, Box<dyn Interactive>, StaticAllocHandle<Vertex>>>,
    buffer: Buffer<Vertex, layout::WidgetAllocator>,
    line_buffer: Buffer<Vertex, layout::WireAllocator>,

    action_queue: std::sync::mpsc::Receiver<ModifyAction<Vertex>>,
    dummy_action_sender: std::sync::mpsc::Sender<ModifyAction<Vertex>>,

    widgets: HashMap<String, WidgetModel>,
}

impl WidgetServer {
    pub fn new(device: &wgpu::Device) -> Self {
        let (action_sender, action_receiver) = std::sync::mpsc::channel();

        Self {
            widget_hitbox: HitboxNode::root(),
            buffer: Buffer::new("Widget Buffer", device),
            line_buffer: Buffer::new("Widget Line Buffer", device),

            action_queue: action_receiver,
            dummy_action_sender: action_sender,

            widgets: HashMap::new(),
        }
    }

    pub fn set_hover_visual(&mut self, visual: Visual<72, 48>, queue: &wgpu::Queue) {
        self.buffer.write(
            "hover_box",
            &rether::SimpleGeometry::init(visual.vertices.to_vec()),
            queue,
        );
        self.line_buffer.write(
            "hover_box",
            &rether::SimpleGeometry::init(visual.wires.to_vec()),
            queue,
        );
    }

    pub fn set_select_visual(&mut self, visual: Visual<72, 48>, queue: &wgpu::Queue) {
        self.buffer.write(
            "select_box",
            &rether::SimpleGeometry::init(visual.vertices.to_vec()),
            queue,
        );
        self.line_buffer.write(
            "select_box",
            &rether::SimpleGeometry::init(visual.wires.to_vec()),
            queue,
        );
    }

    pub fn reset_hover_visual(&mut self, queue: &wgpu::Queue) {
        self.buffer
            .write("hover_box", &rether::SimpleGeometry::empty(), queue);
        self.line_buffer
            .write("hover_box", &rether::SimpleGeometry::empty(), queue);
    }

    pub fn reset_select_visual(&mut self, queue: &wgpu::Queue) {
        self.buffer
            .write("select_box", &rether::SimpleGeometry::empty(), queue);
        self.line_buffer
            .write("select_box", &rether::SimpleGeometry::empty(), queue);
    }

    pub fn read_buffer(&self) -> &Buffer<Vertex, layout::WidgetAllocator> {
        &self.buffer
    }

    pub fn read_line_buffer(&self) -> &Buffer<Vertex, layout::WireAllocator> {
        &self.line_buffer
    }
}
