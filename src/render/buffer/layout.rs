use std::collections::HashMap;

use wgpu::{Device, Queue};

use super::BufferRange;

/*
pub const HOVER_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
    offset: 0,
    size: 12,
};

pub const SELECT_BOX_ALLOCATION: BufferAllocation = BufferAllocation {
    offset: 12,
    size: 12,
};

pub enum BufferLayoutType<T> {
    Static(Box<dyn BufferAlloc<T>>),
    Dynamic(Box<dyn BufferDynamicAlloc<T>>),
}
*/

pub trait BufferAlloc<T> {
    fn write(&self, buffer: &mut super::DynamicBuffer<T>, queue: &Queue, id: &str, data: &[T]);
    fn read<'a>(&self, buffer: &'a mut super::DynamicBuffer<T>, id: &str) -> Option<&'a [T]>;
    fn get(&self, id: &str) -> Option<&BufferAllocation>;
    fn size(&self) -> usize;
}

pub trait BufferDynamicAlloc<T>: BufferAlloc<T> {
    fn allocate(
        &mut self,
        buffer: &mut super::DynamicBuffer<T>,
        device: &Device,
        id: &str,
        size: usize,
    );
    fn allocate_init(
        &mut self,
        buffer: &mut super::DynamicBuffer<T>,
        device: &Device,
        id: &str,
        data: &[T],
    );
    fn free(&mut self, buffer: &mut super::DynamicBuffer<T>, device: &Device, id: &str);
}

#[derive(Debug, Default)]
pub struct BufferDynamicAllocator {
    packets: HashMap<String, BufferAllocation>,
    pub size: usize,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> BufferAlloc<T> for BufferDynamicAllocator {
    fn write(&self, buffer: &mut super::DynamicBuffer<T>, queue: &Queue, id: &str, data: &[T]) {
        if let Some(packet) = self.packets.get(id) {
            buffer.write(queue, packet.offset as u64, data)
        }
    }

    fn read<'a>(&self, buffer: &'a mut super::DynamicBuffer<T>, id: &str) -> Option<&'a [T]> {
        if let Some(packet) = self.packets.get(id) {
            Some(buffer.read(BufferRange::Range(
                packet.offset..(packet.offset + packet.size),
            )))
        } else {
            None
        }
    }

    fn get(&self, id: &str) -> Option<&BufferAllocation> {
        self.packets.get(id)
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> BufferDynamicAlloc<T> for BufferDynamicAllocator {
    fn allocate(
        &mut self,
        buffer: &mut super::DynamicBuffer<T>,
        device: &Device,
        id: &str,
        size: usize,
    ) {
        let offset = self.size;
        self.size += size;
        self.packets
            .insert(id.to_string(), BufferAllocation { offset, size });

        let mut data = buffer.read(BufferRange::Full).to_vec();
        data.extend_from_slice(&vec![T::zeroed(); size]);

        buffer.renew_init(&data, device);
    }

    fn allocate_init(
        &mut self,
        buffer: &mut super::DynamicBuffer<T>,
        device: &Device,
        id: &str,
        data: &[T],
    ) {
        let offset = self.size;
        let size = data.len();
        self.size += size;
        self.packets
            .insert(id.to_string(), BufferAllocation { offset, size });

        let mut buffer_data = buffer.read(BufferRange::Full).to_vec();
        buffer_data.extend_from_slice(data);

        buffer.renew_init(&buffer_data, device);
    }

    fn free(&mut self, buffer: &mut super::DynamicBuffer<T>, device: &Device, id: &str) {
        if let Some(remove_packet) = self.packets.remove(id) {
            self.size -= remove_packet.size;

            // Update offsets of all packets after the removed one
            for packet in self.packets.values_mut() {
                if packet.offset > remove_packet.offset {
                    packet.offset -= remove_packet.size;
                }
            }

            let mut data = buffer.read(BufferRange::Full).to_vec();
            data.drain(remove_packet.offset..(remove_packet.offset + remove_packet.size));

            buffer.renew_init(&data, device);
        }
    }
}

#[derive(Debug)]
pub struct BufferAllocation {
    pub offset: usize,
    pub size: usize,
}
