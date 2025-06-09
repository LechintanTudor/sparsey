use crate::entity::Entity;
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[derive(Default, Debug)]
pub(crate) struct EntityAllocator {
    next_index: AtomicU64,
    last_maintained_index: u64,
    recycled: VecDeque<Entity>,
    recycled_since_maintain: AtomicUsize,
}

impl EntityAllocator {
    #[must_use]
    pub fn allocate(&mut self) -> Option<Entity> {
        debug_assert!(!self.should_maintain_recyled());

        let next_index = self.next_index.get_mut();

        if let Some(entity) = self.recycled.pop_front() {
            Some(entity)
        } else if let Ok(index) = u32::try_from(*next_index) {
            *next_index += 1;
            self.last_maintained_index = *next_index;
            Some(Entity::with_index(index))
        } else {
            None
        }
    }

    #[must_use]
    pub fn allocate_atomic(&self) -> Option<Entity> {
        if let Some(recycled_index) = self.increment_recycled_since_maintain() {
            Some(self.recycled[recycled_index])
        } else {
            self.increment_next_index().map(Entity::with_index)
        }
    }

    pub fn recycle(&mut self, entity: Entity) {
        if let Some(next_version) = entity.version.next() {
            self.recycled
                .push_back(Entity::new(entity.index, next_version));
        }
    }

    #[inline]
    #[must_use]
    pub fn should_maintain_recyled(&mut self) -> bool {
        *self.recycled_since_maintain.get_mut() != 0
    }

    pub fn maintain_recycled(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let recyled_since_maintain = *self.recycled_since_maintain.get_mut();
        *self.recycled_since_maintain.get_mut() = 0;
        self.recycled.drain(..recyled_since_maintain)
    }

    pub fn maintain_new(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let next_index = *self.next_index.get_mut();
        let last_maintained_index = self.last_maintained_index;
        self.last_maintained_index = next_index;

        (last_maintained_index..next_index).map(|i| Entity::with_index(i as u32))
    }

    pub fn reset(&mut self) {
        *self.next_index.get_mut() = 0;
        self.last_maintained_index = 0;
        self.recycled.clear();
        *self.recycled_since_maintain.get_mut() = 0;
    }

    fn increment_next_index(&self) -> Option<u32> {
        let mut prev = self.next_index.load(Ordering::Relaxed);

        while u32::try_from(prev).is_ok() {
            match self.next_index.compare_exchange_weak(
                prev,
                prev + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(prev) => return Some(prev as u32),
                Err(next_prev) => prev = next_prev,
            }
        }

        None
    }

    fn increment_recycled_since_maintain(&self) -> Option<usize> {
        let recycled_len = self.recycled.len();
        let mut prev = self.recycled_since_maintain.load(Ordering::Relaxed);

        while prev < recycled_len {
            match self.recycled_since_maintain.compare_exchange_weak(
                prev,
                prev + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(prev) => return Some(prev),
                Err(next_prev) => prev = next_prev,
            }
        }

        None
    }
}
