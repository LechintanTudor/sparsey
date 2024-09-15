use crate::entity::{Entity, SparseVec, SparseVecSlot};
use alloc::vec::Vec;
use core::{fmt, mem};

#[derive(Clone, Default)]
pub(crate) struct EntitySparseSet {
    sparse: SparseVec,
    entities: Vec<Entity>,
}

impl EntitySparseSet {
    pub fn insert(&mut self, entity: Entity) -> Option<Entity> {
        let dense_entity = self.sparse.get_mut_or_allocate_at(entity.sparse());

        match dense_entity {
            Some(dense_entity) => unsafe {
                Some(mem::replace(
                    self.entities.get_unchecked_mut(dense_entity.index as usize),
                    entity,
                ))
            },
            None => {
                *dense_entity = Some(SparseVecSlot {
                    index: self.entities.len() as u32,
                    version: entity.version,
                });

                self.entities.push(entity);
                None
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        let Some(raw_dense) = self.sparse.remove(entity) else {
            return false;
        };

        let dense = raw_dense as usize;
        self.entities.swap_remove(dense);

        if let Some(entity) = self.entities.get(dense) {
            unsafe {
                *self.sparse.get_unchecked_mut(entity.sparse()) = Some(SparseVecSlot {
                    index: raw_dense,
                    version: entity.version,
                });
            }
        }

        true
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Entity] {
        &self.entities
    }

    #[inline]
    pub fn clear(&mut self) {
        self.sparse.clear();
        self.entities.clear();
    }
}

impl fmt::Debug for EntitySparseSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(&self.entities).finish()
    }
}
