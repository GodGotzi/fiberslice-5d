use std::collections::BinaryHeap;

use super::hitbox::HitboxNode;

pub type HitboxQueue<'a, C> = BinaryHeap<HitBoxQueueEntry<'a, C>>;

#[derive(Debug)]
pub struct HitBoxQueueEntry<'a, C> {
    pub hitbox: &'a HitboxNode<C>,
    pub distance: f32,
}

impl<C> PartialEq for HitBoxQueueEntry<'_, C> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<C> Eq for HitBoxQueueEntry<'_, C> {}

impl<C> PartialOrd for HitBoxQueueEntry<'_, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for HitBoxQueueEntry<'_, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
