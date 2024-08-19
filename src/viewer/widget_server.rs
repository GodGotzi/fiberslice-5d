use std::collections::HashMap;

use rether::{
    alloc::{ModifyAction, StaticAllocHandle},
    model::{BaseModel, TreeModel},
    picking::{interact::Interactive, HitboxNode, HitboxRoot},
    vertex::Vertex,
    Buffer, Rotate, Scale, Translate,
};

use super::Visual;

mod layout {

    mod wire {
        use std::sync::Arc;

        use rether::{
            alloc::{BufferAlloc, BufferAllocation, StaticAllocHandle},
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
        pub struct WireAllocator {
            action_queue: std::sync::mpsc::Receiver<rether::alloc::ModifyAction<Vertex>>,
            dummy_action_sender: std::sync::mpsc::Sender<rether::alloc::ModifyAction<Vertex>>,

            hover_box: Arc<StaticAllocHandle<Vertex>>,
            select_box: Arc<StaticAllocHandle<Vertex>>,
        }

        impl Default for WireAllocator {
            fn default() -> Self {
                let (action_sender, action_receiver) = std::sync::mpsc::channel();

                Self {
                    action_queue: action_receiver,
                    dummy_action_sender: action_sender.clone(),

                    hover_box: Arc::new(StaticAllocHandle::from_buffer_allocation(
                        "hover_box",
                        &HOVER_BOX_ALLOCATION,
                        action_sender.clone(),
                    )),
                    select_box: Arc::new(StaticAllocHandle::from_buffer_allocation(
                        "select_box",
                        &SELECT_BOX_ALLOCATION,
                        action_sender.clone(),
                    )),
                }
            }
        }

        impl BufferAlloc<Vertex> for WireAllocator {
            type Handle = StaticAllocHandle<Vertex>;

            fn get(&self, id: &str) -> Option<&Arc<StaticAllocHandle<Vertex>>> {
                match id {
                    "hover_box" => Some(&self.hover_box),
                    "select_box" => Some(&self.select_box),
                    _ => None,
                }
            }

            fn size(&self) -> usize {
                HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size
            }

            fn update(&self, modify: impl Fn(rether::alloc::ModifyAction<Vertex>)) {
                while let Ok(action) = self.action_queue.try_recv() {
                    modify(action);
                }
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

    use std::sync::Arc;

    use rether::{
        alloc::{BufferAlloc, BufferAllocation, StaticAllocHandle},
        vertex::Vertex,
    };
    pub use wire::WireAllocator;

    #[derive(Debug)]
    pub struct VertexAllocator {
        action_queue: std::sync::mpsc::Receiver<rether::alloc::ModifyAction<Vertex>>,
        dummy_action_sender: std::sync::mpsc::Sender<rether::alloc::ModifyAction<Vertex>>,

        hover_box: Arc<StaticAllocHandle<Vertex>>,
        select_box: Arc<StaticAllocHandle<Vertex>>,
    }

    impl Default for VertexAllocator {
        fn default() -> Self {
            let (action_sender, action_receiver) = std::sync::mpsc::channel();

            Self {
                action_queue: action_receiver,
                dummy_action_sender: action_sender.clone(),

                hover_box: Arc::new(StaticAllocHandle::from_buffer_allocation(
                    "hover_box",
                    &HOVER_BOX_ALLOCATION,
                    action_sender.clone(),
                )),
                select_box: Arc::new(StaticAllocHandle::from_buffer_allocation(
                    "select_box",
                    &SELECT_BOX_ALLOCATION,
                    action_sender.clone(),
                )),
            }
        }
    }

    impl BufferAlloc<Vertex> for VertexAllocator {
        type Handle = StaticAllocHandle<Vertex>;

        fn get(&self, id: &str) -> Option<&std::sync::Arc<Self::Handle>> {
            match id {
                "hover_box" => Some(&self.hover_box),
                "select_box" => Some(&self.select_box),
                _ => None,
            }
        }

        fn size(&self) -> usize {
            HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size
        }

        fn update(&self, modify: impl Fn(rether::alloc::ModifyAction<Vertex>)) {
            while let Ok(action) = self.action_queue.try_recv() {
                modify(action);
            }
        }
    }
}

pub trait WidgetContextImpl: Translate + Scale + Rotate + std::fmt::Debug {}

pub struct WidgetContext {
    ctx: Box<dyn Interactive>,
}

#[derive(Debug)]
pub struct WidgetModel {
    handle: TreeModel<Vertex, WidgetContext, StaticAllocHandle<Vertex>>,
}

#[derive(Debug)]
pub struct WidgetServer {
    widget_hitbox: HitboxRoot<BaseModel<Vertex, Box<dyn Interactive>, StaticAllocHandle<Vertex>>>,
    buffer: Buffer<Vertex, layout::VertexAllocator>,
    line_buffer: Buffer<Vertex, layout::WireAllocator>,

    action_queue: std::sync::mpsc::Receiver<ModifyAction<Vertex>>,
    dummy_action_sender: std::sync::mpsc::Sender<ModifyAction<Vertex>>,

    widgets: HashMap<String, WidgetModel>,
}

impl WidgetServer {
    pub fn new(device: &wgpu::Device) -> Self {
        let (action_sender, action_receiver) = std::sync::mpsc::channel();

        Self {
            widget_hitbox: HitboxRoot::root(),
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

    pub fn read_buffer(&self) -> &Buffer<Vertex, layout::VertexAllocator> {
        &self.buffer
    }

    pub fn read_line_buffer(&self) -> &Buffer<Vertex, layout::WireAllocator> {
        &self.line_buffer
    }
}
