use std::collections::BinaryHeap;

use super::hitbox::HitboxNode;

pub type HitboxQueue<'a, C> = BinaryHeap<HitBoxQueueEntry<'a, C>>;

#[derive(Debug)]
pub struct HitBoxQueueEntry<'a, M> {
    pub hitbox: &'a M,
    pub distance: f32,
}

impl<M: HitboxNode<M>> PartialEq for HitBoxQueueEntry<'_, M> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<M: HitboxNode<M>> Eq for HitBoxQueueEntry<'_, M> {}

impl<M: HitboxNode<M>> PartialOrd for HitBoxQueueEntry<'_, M> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: HitboxNode<M>> Ord for HitBoxQueueEntry<'_, M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap()
            .reverse()
    }
}
