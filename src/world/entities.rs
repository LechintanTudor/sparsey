use crate::storage::{Entity, EntityStorage};

#[derive(Clone, Copy)]
pub struct Entities<'a> {
    storage: &'a EntityStorage,
}

impl<'a> Entities<'a> {
    pub(crate) fn new(storage: &'a EntityStorage) -> Self {
        Self { storage }
    }

    #[inline]
    pub fn create(&self) -> Entity {
        self.storage.create_atomic()
    }

    #[inline]
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    #[inline]
    pub fn as_slice(&self) -> &[Entity] {
        self.storage
    }
}
