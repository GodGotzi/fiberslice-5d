use crate::{geometry::BoundingBox, render::buffer::BufferLocation};

#[derive(Debug)]
pub struct InteractiveMesh<C> {
    location: BufferLocation,
    raw_box: BoundingBox,
    context: C,
}
