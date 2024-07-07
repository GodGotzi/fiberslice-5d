use crate::geometry::SelectBox;

use super::alloc::BufferAllocation;

pub mod wire {
    use crate::{
        geometry::SelectBox,
        render::buffer::alloc::{BufferAlloc, BufferAllocation},
    };

    const HOVER_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
        offset: 0,
        size: SelectBox::wire_vertex_count(),
    };

    const SELECT_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
        offset: HOVER_BOX_ALLOCATION.size,
        size: SelectBox::wire_vertex_count(),
    };

    const RAY_DEBUG_ALLOCATION: BufferAllocation = BufferAllocation {
        offset: HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size,
        size: 4,
    };

    #[derive(Debug)]
    pub struct WireAllocator;

    impl<T> BufferAlloc<T> for WireAllocator {
        fn get(&self, id: &str) -> Option<&BufferAllocation> {
            match id {
                "hover_box" => Some(&HOVER_BOX_ALLOCATION),
                "select_box" => Some(&SELECT_BOX_ALLOCATION),
                "ray_debug" => Some(&RAY_DEBUG_ALLOCATION),
                _ => None,
            }
        }

        fn size(&self) -> usize {
            HOVER_BOX_ALLOCATION.size + SELECT_BOX_ALLOCATION.size + RAY_DEBUG_ALLOCATION.size
        }
    }
}

const HOVER_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
    offset: 0,
    size: SelectBox::traingle_vertex_count(),
};

const SELECT_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
    offset: HOVER_BOX_ALLOCATION.size,
    size: SelectBox::traingle_vertex_count(),
};

#[derive(Debug)]
pub struct WidgetAllocator;

impl<T> super::alloc::BufferAlloc<T> for WidgetAllocator {
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
