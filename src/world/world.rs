use crate::storage::{Entities, Entity};
use crate::world::{
    BorrowSparseSetSet, Comp, CompMut, Component, ComponentSet, Components, SparseSetRefMut,
    WorldLayoutDescriptor,
};
use std::borrow::Borrow;
use std::collections::HashSet;

pub struct World {
    entities: Entities,
    components: Components,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Entities::default(),
            components: Components::new(&L::world_layout()),
        }
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.components.register::<T>();
    }

    pub fn maintain(&mut self) {
        self.entities.maintain();

        unsafe {
            for sparse_set in self.components.iter_sparse_sets_mut() {
                sparse_set.maintain();
            }
        }
    }

    pub fn create_entity<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        self.append_components(entity, components).unwrap();
        entity
    }

    pub fn extend<C, I>(&mut self, _component_iter: I) -> ()
    where
        C: ComponentSet,
        I: Iterator<Item = C>,
    {
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity) {
            return false;
        }

        unsafe {
            {
                let mut group_set = self.components.get_full_group_set();
                group_set.ungroup_components(entity);
            }

            for sparse_set in self.components.iter_sparse_sets_mut() {
                sparse_set.delete(entity);
            }
        }

        true
    }

    pub fn append_components<C>(&mut self, entity: Entity, components: C) -> Result<(), ()>
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return Err(());
        }

        unsafe {
            {
                let mut sparse_set_set = <C::Borrow as BorrowSparseSetSet>::borrow(self);
                C::append(&mut sparse_set_set, entity, components);
            }

            let group_indexes = C::components()
                .as_ref()
                .iter()
                .flat_map(|type_id| self.components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            if !group_indexes.is_empty() {
                let mut group_set = self.components.get_group_set(&group_indexes);
                group_set.group_components(entity);
            }
        }

        Ok(())
    }

    pub fn remove_components<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return None;
        }

        unsafe {
            let group_indexes = C::components()
                .as_ref()
                .iter()
                .flat_map(|type_id| self.components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            if !group_indexes.is_empty() {
                let mut group_set = self.components.get_group_set(&group_indexes);
                group_set.ungroup_components(entity);
            }

            {
                let mut sparse_set_set = <C::Borrow as BorrowSparseSetSet>::borrow(self);
                C::remove(&mut sparse_set_set, entity)
            }
        }
    }

    pub fn delete_components<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return;
        }

        unsafe {
            let group_indexes = C::components()
                .as_ref()
                .iter()
                .flat_map(|type_id| self.components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            if !group_indexes.is_empty() {
                let mut group_set = self.components.get_group_set(&group_indexes);
                group_set.ungroup_components(entity);
            }

            {
                let mut sparse_set_set = <C::Borrow as BorrowSparseSetSet>::borrow(self);
                C::delete(&mut sparse_set_set, entity)
            }
        }
    }

    pub fn drain<C, E, I>(&mut self, _entities: I) -> ()
    where
        C: ComponentSet,
        E: Borrow<Entity>,
        I: Iterator<Item = E>,
    {
        todo!()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        self.components.borrow_comp::<T>()
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        self.components.borrow_comp_mut::<T>()
    }

    pub(crate) unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<SparseSetRefMut<T>>
    where
        T: Component,
    {
        self.components.borrow_sparse_set_mut::<T>()
    }

    pub(crate) fn entities(&self) -> &Entities {
        &self.entities
    }
}
