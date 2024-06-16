use std::collections::BinaryHeap;

use super::hitbox::HitboxNode;

pub type HitboxQueue<'a> = BinaryHeap<HitBoxQueueEntry<'a>>;

#[derive(Debug)]
pub struct HitBoxQueueEntry<'a> {
    pub hitbox: &'a HitboxNode,
    pub distance: f32,
}

impl PartialEq for HitBoxQueueEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for HitBoxQueueEntry<'_> {}

impl PartialOrd for HitBoxQueueEntry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HitBoxQueueEntry<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
