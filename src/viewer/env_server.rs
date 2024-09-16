use std::fmt::Debug;

use rether::{
    alloc::StaticAllocHandle,
    model::{geometry::Geometry, BaseModel, Model},
    vertex::Vertex,
    Buffer,
};

use super::volume::Volume;

mod layout {

    mod line {
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

        const VOLUME_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
            offset: HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size,
            size: 48,
        };

        #[derive(Debug)]
        pub struct CoverLineAllocator {
            action_queue: std::sync::mpsc::Receiver<rether::alloc::ModifyAction<Vertex>>,
            dummy_action_sender: std::sync::mpsc::Sender<rether::alloc::ModifyAction<Vertex>>,

            hover_box: Arc<StaticAllocHandle<Vertex>>,
            select_box: Arc<StaticAllocHandle<Vertex>>,
            volume_box: Arc<StaticAllocHandle<Vertex>>,
        }

        impl Default for CoverLineAllocator {
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
                    volume_box: Arc::new(StaticAllocHandle::from_buffer_allocation(
                        "volume_box",
                        &VOLUME_BOX_ALLOCATION,
                        action_sender.clone(),
                    )),
                }
            }
        }

        impl BufferAlloc<Vertex> for CoverLineAllocator {
            type Handle = StaticAllocHandle<Vertex>;

            fn get(&self, id: &str) -> Option<&Arc<StaticAllocHandle<Vertex>>> {
                match id {
                    "hover_box" => Some(&self.hover_box),
                    "select_box" => Some(&self.select_box),
                    "volume_box" => Some(&self.volume_box),
                    _ => None,
                }
            }

            fn size(&self) -> usize {
                HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size + VOLUME_BOX_ALLOCATION.size
            }

            fn update(&self, modify: impl Fn(rether::alloc::ModifyAction<Vertex>)) {
                while let Ok(action) = self.action_queue.try_recv() {
                    modify(action);
                }
            }
        }

        const GRID_ALLOCATION: BufferAllocation = BufferAllocation {
            offset: 0,
            size: 88,
        };

        #[derive(Debug)]
        pub struct LineAllocator {
            action_queue: std::sync::mpsc::Receiver<rether::alloc::ModifyAction<Vertex>>,
            dummy_action_sender: std::sync::mpsc::Sender<rether::alloc::ModifyAction<Vertex>>,

            grid: Arc<StaticAllocHandle<Vertex>>,
        }

        impl Default for LineAllocator {
            fn default() -> Self {
                let (action_sender, action_receiver) = std::sync::mpsc::channel();

                Self {
                    action_queue: action_receiver,
                    dummy_action_sender: action_sender.clone(),

                    grid: Arc::new(StaticAllocHandle::from_buffer_allocation(
                        "grid",
                        &GRID_ALLOCATION,
                        action_sender.clone(),
                    )),
                }
            }
        }

        impl BufferAlloc<Vertex> for LineAllocator {
            type Handle = StaticAllocHandle<Vertex>;

            fn get(&self, id: &str) -> Option<&Arc<StaticAllocHandle<Vertex>>> {
                match id {
                    "grid" => Some(&self.grid),
                    _ => None,
                }
            }

            fn size(&self) -> usize {
                GRID_ALLOCATION.size
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

    const BED_ALLOCATION: BufferAllocation = BufferAllocation {
        offset: HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size,
        size: 6,
    };

    use std::sync::Arc;

    pub use line::CoverLineAllocator;
    pub use line::LineAllocator;
    use rether::{
        alloc::{BufferAlloc, BufferAllocation, StaticAllocHandle},
        vertex::Vertex,
    };

    #[derive(Debug)]
    pub struct VertexAllocator {
        action_queue: std::sync::mpsc::Receiver<rether::alloc::ModifyAction<Vertex>>,
        dummy_action_sender: std::sync::mpsc::Sender<rether::alloc::ModifyAction<Vertex>>,

        hover_box: Arc<StaticAllocHandle<Vertex>>,
        select_box: Arc<StaticAllocHandle<Vertex>>,
        volume_box: Arc<StaticAllocHandle<Vertex>>,
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
                volume_box: Arc::new(StaticAllocHandle::from_buffer_allocation(
                    "bed",
                    &BED_ALLOCATION,
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
                "bed" => Some(&self.volume_box),
                _ => None,
            }
        }

        fn size(&self) -> usize {
            HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size + BED_ALLOCATION.size
        }

        fn update(&self, modify: impl Fn(rether::alloc::ModifyAction<Vertex>)) {
            while let Ok(action) = self.action_queue.try_recv() {
                modify(action);
            }
        }
    }
}

#[derive(Debug)]
pub struct EnvironmentServer {
    buffer: Buffer<Vertex, layout::VertexAllocator>,
    line_buffer: Buffer<Vertex, layout::LineAllocator>,
    line_buffer_cover: Buffer<Vertex, layout::CoverLineAllocator>,

    volume: Volume,
}

impl EnvironmentServer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let server = Self {
            buffer: Buffer::new("Widget Buffer", device),

            line_buffer: Buffer::new("Widget Line Buffer", device),
            line_buffer_cover: Buffer::new("Widget Line Buffer Cover", device),

            volume: Volume::default(),
        };

        server.init_line_widget_cover("volume_box", &server.volume.r#box, queue);
        server.init_line_widget("grid", &server.volume.grid_model, queue);
        server.init_widget("bed", &server.volume.bed, queue);

        server
    }

    pub fn init_widget(
        &self,
        id: &str,
        model: &BaseModel<Vertex, StaticAllocHandle<Vertex>>,
        queue: &wgpu::Queue,
    ) {
        match &*model.state().read() {
            rether::model::ModelState::Dormant(geometry) => {
                self.buffer.write(id, geometry.build_data(), queue)
            }
            _ => panic!("Unsupported geometry"),
        };

        model.wake(self.buffer.get(id).unwrap().clone());
    }

    pub fn init_line_widget(
        &self,
        id: &str,
        model: &BaseModel<Vertex, StaticAllocHandle<Vertex>>,
        queue: &wgpu::Queue,
    ) {
        match &*model.state().read() {
            rether::model::ModelState::Dormant(geometry) => {
                self.line_buffer.write(id, geometry.build_data(), queue)
            }
            _ => panic!("Unsupported geometry"),
        };

        model.wake(self.line_buffer.get(id).unwrap().clone());
    }

    pub fn init_line_widget_cover(
        &self,
        id: &str,
        model: &BaseModel<Vertex, StaticAllocHandle<Vertex>>,
        queue: &wgpu::Queue,
    ) {
        match &*model.state().read() {
            rether::model::ModelState::Dormant(geometry) => {
                self.line_buffer_cover
                    .write(id, geometry.build_data(), queue)
            }
            _ => panic!("Unsupported geometry"),
        };

        model.wake(self.line_buffer_cover.get(id).unwrap().clone());
    }

    pub fn read_buffer(&self) -> &Buffer<Vertex, layout::VertexAllocator> {
        &self.buffer
    }

    pub fn read_line_buffer(&self) -> &Buffer<Vertex, layout::LineAllocator> {
        &self.line_buffer
    }

    pub fn read_line_cover_buffer(&self) -> &Buffer<Vertex, layout::CoverLineAllocator> {
        &self.line_buffer_cover
    }
}
