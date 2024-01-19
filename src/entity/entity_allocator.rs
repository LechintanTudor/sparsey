use crate::entity::Entity;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[derive(Default, Debug)]
pub(crate) struct EntityAllocator {
    next_index_to_allocate: AtomicU64,
    last_maintained_index: u64,
    recycled: VecDeque<Entity>,
    recycled_since_maintain: AtomicUsize,
}

impl EntityAllocator {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            next_index_to_allocate: AtomicU64::new(0),
            last_maintained_index: 0,
            recycled: VecDeque::new(),
            recycled_since_maintain: AtomicUsize::new(0),
        }
    }

    #[must_use]
    pub fn allocate(&mut self) -> Option<Entity> {
        let recycled_since_maintain = *self.recycled_since_maintain.get_mut();

        if recycled_since_maintain < self.recycled.len() {
            *self.recycled_since_maintain.get_mut() += 1;
            Some(self.recycled[self.recycled.len() - recycled_since_maintain - 1])
        } else if let Ok(index) = u32::try_from(*self.next_index_to_allocate.get_mut()) {
            *self.next_index_to_allocate.get_mut() += 1;
            Some(Entity::with_index(index))
        } else {
            None
        }
    }

    #[must_use]
    pub fn allocate_atomic(&self) -> Option<Entity> {
        match self.increment_recycled_since_maintain() {
            Some(recycled_since_maintain) => {
                Some(self.recycled[self.recycled.len() - recycled_since_maintain - 1])
            }
            None => {
                self.increment_next_index_to_allocate()
                    .map(Entity::with_index)
            }
        }
    }

    pub fn recycle(&mut self, entity: Entity) {
        if let Some(next_version) = entity.version.next() {
            self.recycled
                .push_front(Entity::new(entity.index, next_version));
        }
    }

    pub fn maintain(&mut self) -> impl Iterator<Item = Entity> + '_ {
        let recycled_range = {
            let recycled_since_maintain = *self.recycled_since_maintain.get_mut();
            *self.recycled_since_maintain.get_mut() = 0;
            (self.recycled.len() - recycled_since_maintain)..
        };

        let new_index_range = {
            let next_index_to_allocate = *self.next_index_to_allocate.get_mut();
            let new_index_range = self.last_maintained_index..next_index_to_allocate;
            self.last_maintained_index = next_index_to_allocate;
            new_index_range
        };

        self.recycled
            .drain(recycled_range)
            .chain(new_index_range.map(|i| Entity::with_index(i as u32)))
    }

    pub fn reset(&mut self) {
        *self.next_index_to_allocate.get_mut() = 0;
        self.last_maintained_index = 0;
        self.recycled.clear();
        *self.recycled_since_maintain.get_mut() = 0;
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

    fn increment_next_index_to_allocate(&self) -> Option<u32> {
        let mut prev = self.next_index_to_allocate.load(Ordering::Relaxed);

        while u32::try_from(prev).is_ok() {
            match self.next_index_to_allocate.compare_exchange_weak(
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
}
