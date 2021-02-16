use crate::data::{Component, ComponentFlags, ComponentRefMut, Entity, IndexEntity, SparseArray};

pub struct SparseSetMutPtr<T>
where
    T: Component,
{
    sparse: *mut SparseArray,
    dense: *mut Vec<Entity>,
    flags: *mut Vec<ComponentFlags>,
    data: *mut Vec<T>,
}

impl<T> SparseSetMutPtr<T>
where
    T: Component,
{
    pub unsafe fn new(
        sparse: &mut SparseArray,
        dense: &mut Vec<Entity>,
        flags: &mut Vec<ComponentFlags>,
        data: &mut Vec<T>,
    ) -> Self {
        Self {
            sparse: sparse as _,
            dense: dense as _,
            flags: flags as _,
            data: data as _,
        }
    }

    pub unsafe fn insert(&mut self, entity: Entity, value: T) {
        let index_entity = (*self.sparse).get_mut_or_allocate(entity.index());

        match index_entity {
            Some(e) => {
                if e.id() == entity.id() {
                    (*self.flags)[e.index()].insert(ComponentFlags::CHANGED);
                } else {
                    (*self.flags)[e.index()].insert(ComponentFlags::ADDED);
                }

                *e = IndexEntity::new(e.id(), entity.ver());
                (*self.data)[e.index()] = value;
            }
            None => {
                *index_entity = Some(IndexEntity::new((*self.dense).len() as u32, entity.ver()));
                (*self.dense).push(entity);
                (*self.data).push(value);
                (*self.flags).push(ComponentFlags::ADDED);
            }
        }
    }

    pub unsafe fn remove(&mut self, entity: Entity) -> Option<T> {
        let index_entity = (*self.sparse).get_index_entity(entity)?;

        let last_index = (*self.dense).last()?.index();
        (*self.dense).swap_remove(index_entity.index());
        (*self.flags).swap_remove(index_entity.index());

        *(*self.sparse).get_unchecked_mut(last_index) = Some(index_entity);
        *(*self.sparse).get_unchecked_mut(entity.index()) = None;

        Some((*self.data).swap_remove(index_entity.index()))
    }

    pub unsafe fn get(&self, entity: Entity) -> Option<&T> {
        let index = (*self.sparse).get_index_entity(entity)?.index();
        Some((*self.data).get_unchecked(index))
    }

    pub unsafe fn get_mut(&mut self, entity: Entity) -> Option<ComponentRefMut<T>> {
        let index = (*self.sparse).get_index_entity(entity)?.index();
        Some(ComponentRefMut::new(
            (*self.data).get_unchecked_mut(index),
            (*self.flags).get_unchecked_mut(index),
        ))
    }
}
