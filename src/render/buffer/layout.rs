use std::collections::HashMap;

pub struct BufferAllocator {
    packets: HashMap<String, BufferAllocation>,
    pub size: usize,
}

impl BufferAllocator {
    pub fn new() -> Self {
        BufferAllocator {
            packets: HashMap::new(),
            size: 0,
        }
    }

    pub fn allocate(&mut self, name: &str, size: usize) {
        let offset = self.size;
        self.size += size;
        self.packets
            .insert(name.to_string(), BufferAllocation { offset, size });
    }

    pub fn free(&mut self, name: &str) {
        if let Some(remove_packet) = self.packets.remove(name) {
            self.size -= remove_packet.size;

            // Update offsets of all packets after the removed one
            for packet in self.packets.values_mut() {
                if packet.offset > remove_packet.offset {
                    packet.offset -= remove_packet.size;
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&BufferAllocation> {
        self.packets.get(name)
    }
}

pub struct BufferAllocation {
    pub offset: usize,
    pub size: usize,
}
