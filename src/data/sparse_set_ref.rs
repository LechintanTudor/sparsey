use crate::data::{Component, ComponentFlags, ComponentRefMut, Entity, IndexEntity, SparseArray};

pub struct SparseSetRef<'a, T>
where
    T: Component,
{
    sparse: &'a SparseArray,
    dense: &'a [Entity],
    flags: &'a [ComponentFlags],
    data: &'a [T],
}

impl<'a, T> SparseSetRef<'a, T>
where
    T: Component,
{
    pub unsafe fn new(
        sparse: &'a SparseArray,
        dense: &'a [Entity],
        flags: &'a [ComponentFlags],
        data: &'a [T],
    ) -> Self {
        Self {
            sparse,
            dense,
            data,
            flags,
        }
    }

    pub fn split(&self) -> (&SparseArray, &[Entity], &[ComponentFlags], &[T]) {
        (self.sparse, self.dense, self.flags, self.data)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get_index_entity(entity)?.index();
        unsafe { Some(self.data.get_unchecked(index)) }
    }
}

pub struct SparseSetRefMut<'a, T>
where
    T: Component,
{
    sparse: &'a mut SparseArray,
    dense: &'a mut Vec<Entity>,
    flags: &'a mut Vec<ComponentFlags>,
    data: &'a mut Vec<T>,
}

impl<'a, T> SparseSetRefMut<'a, T>
where
    T: Component,
{
    pub unsafe fn new(
        sparse: &'a mut SparseArray,
        dense: &'a mut Vec<Entity>,
        flags: &'a mut Vec<ComponentFlags>,
        data: &'a mut Vec<T>,
    ) -> Self {
        Self {
            sparse,
            dense,
            flags,
            data,
        }
    }

    pub fn insert(&mut self, entity: Entity, value: T) {
        let index_entity = self.sparse.get_mut_or_allocate(entity.index());

        match index_entity {
            Some(e) => {
                if e.id() == entity.id() {
                    self.flags[e.index()].insert(ComponentFlags::CHANGED);
                } else {
                    self.flags[e.index()].insert(ComponentFlags::ADDED);
                }

                *e = IndexEntity::new(e.id(), entity.ver());
                self.data[e.index()] = value;
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.dense.len() as u32, entity.ver()));
                self.dense.push(entity);
                self.data.push(value);
                self.flags.push(ComponentFlags::ADDED);
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let index_entity = self.sparse.get_index_entity(entity)?;

        let last_index = self.dense.last()?.index();
        self.dense.swap_remove(index_entity.index());
        self.flags.swap_remove(index_entity.index());

        unsafe {
            *self.sparse.get_unchecked_mut(last_index) = Some(index_entity);
            *self.sparse.get_unchecked_mut(entity.index()) = None;
        }

        Some(self.data.swap_remove(index_entity.index()))
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get_index_entity(entity)?.index();
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<ComponentRefMut<T>> {
        let index = self.sparse.get_index_entity(entity)?.index();
        unsafe {
            Some(ComponentRefMut::new(
                self.data.get_unchecked_mut(index),
                self.flags.get_unchecked_mut(index),
            ))
        }
    }

    pub fn split(&self) -> (&SparseArray, &[Entity], &[ComponentFlags], &[T]) {
        (
            self.sparse,
            self.dense.as_slice(),
            self.flags.as_slice(),
            self.data.as_slice(),
        )
    }

    pub fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [ComponentFlags], &mut [T]) {
        (
            self.sparse,
            self.dense.as_slice(),
            self.flags.as_mut_slice(),
            self.data.as_mut_slice(),
        )
    }
}
