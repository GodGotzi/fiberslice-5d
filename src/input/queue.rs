use std::{collections::BinaryHeap, sync::Arc};

use super::hitbox::HitboxNode;

pub type HitboxQueue<C> = BinaryHeap<HitBoxQueueEntry<C>>;

#[derive(Debug)]
pub struct HitBoxQueueEntry<M> {
    pub hitbox: Arc<M>,
    pub distance: f32,
    pub level: usize,
}

impl<M: HitboxNode<M>> PartialEq for HitBoxQueueEntry<M> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<M: HitboxNode<M>> Eq for HitBoxQueueEntry<M> {}

impl<M: HitboxNode<M>> PartialOrd for HitBoxQueueEntry<M> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: HitboxNode<M>> Ord for HitBoxQueueEntry<M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap()
            .reverse()
    }
}
