use std::collections::HashMap;

pub struct BufferLayout {
    packets: HashMap<String, BufferAllocation>,
}

pub struct BufferAllocation {
    offset: usize,
    size: usize,
}
