use crate::storage::{AbstractSparseSet, Entities, Entity, SparseSet};
use crate::world::{
    BorrowSparseSetSet, Comp, CompMut, Component, ComponentSet, ComponentTypeId, GroupedComponents,
    SparseSetRefMut, UngroupedComponents, WorldLayoutDescriptor,
};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hint::unreachable_unchecked;

pub struct World {
    entities: Entities,
    grouped_components: GroupedComponents,
    ungrouped_components: UngroupedComponents,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Default::default(),
            grouped_components: GroupedComponents::new(&L::world_layout()),
            ungrouped_components: UngroupedComponents::default(),
        }
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        if !self
            .grouped_components
            .contains(&ComponentTypeId::of::<C>())
        {
            self.ungrouped_components.register::<C>();
        }
    }

    pub fn maintain(&mut self) {
        self.entities.maintain();

        unsafe {
            for sparse_set in self.grouped_components.iter_sparse_sets_mut() {
                sparse_set.maintain();
            }

            for sparse_set in self.ungrouped_components.iter_sparse_sets_mut() {
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

    pub fn extend<C, I>(&mut self, component_iter: I) -> ()
    where
        C: ComponentSet,
        I: Iterator<Item = C>,
    {
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        if !self.entities.contains(entity) {
            return false;
        }

        unsafe {
            let mut group_set = self.grouped_components.get_full_group_set();
            group_set.ungroup_components(entity);

            for sparse_set in self.grouped_components.iter_sparse_sets_mut() {
                sparse_set.delete(entity);
            }
            for sparse_set in self.ungrouped_components.iter_sparse_sets_mut() {
                sparse_set.delete(entity);
            }
        }

        self.entities.destroy(entity);
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
                .flat_map(|type_id| self.grouped_components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            if !group_indexes.is_empty() {
                let mut group_set = self.grouped_components.get_group_set(&group_indexes);
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
                .flat_map(|type_id| self.grouped_components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            if !group_indexes.is_empty() {
                let mut group_set = self.grouped_components.get_group_set(&group_indexes);
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
                .flat_map(|type_id| self.grouped_components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            if !group_indexes.is_empty() {
                let mut group_set = self.grouped_components.get_group_set(&group_indexes);
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
        self.grouped_components.clear();
        self.ungrouped_components.clear();
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        let type_id = ComponentTypeId::of::<T>();

        match self.grouped_components.borrow_abstract(&type_id) {
            Some(sparse_set) => unsafe {
                Some(Comp::new(
                    downcast_sparse_set::<T>(sparse_set),
                    self.grouped_components.get_group(&type_id),
                ))
            },
            None => {
                let sparse_set = self.ungrouped_components.borrow_abstract(&type_id)?;

                unsafe { Some(Comp::new(downcast_sparse_set::<T>(sparse_set), None)) }
            }
        }
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        let type_id = ComponentTypeId::of::<T>();

        match unsafe { self.grouped_components.borrow_abstract_mut(&type_id) } {
            Some(sparse_set) => unsafe {
                Some(CompMut::new(
                    downcast_sparse_set_mut::<T>(sparse_set),
                    self.grouped_components.get_group(&type_id),
                ))
            },
            None => {
                let sparse_set = self.ungrouped_components.borrow_abstract_mut(&type_id)?;

                unsafe { Some(CompMut::new(downcast_sparse_set_mut::<T>(sparse_set), None)) }
            }
        }
    }
    pub(crate) unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<SparseSetRefMut<T>>
    where
        T: Component,
    {
        let type_id = ComponentTypeId::of::<T>();
        let sparse_set = self
            .grouped_components
            .borrow_abstract_mut(&type_id)
            .or_else(|| self.ungrouped_components.borrow_abstract_mut(&type_id))?;

        Some(SparseSetRefMut::new(downcast_sparse_set_mut::<T>(
            sparse_set,
        )))
    }

    pub(crate) fn entities(&self) -> &Entities {
        &self.entities
    }
}

unsafe fn downcast_sparse_set<T>(
    sparse_set: AtomicRef<dyn AbstractSparseSet>,
) -> AtomicRef<SparseSet<T>>
where
    T: Component,
{
    AtomicRef::map(sparse_set, |sparse_set| {
        match sparse_set.downcast_ref::<SparseSet<T>>() {
            Some(sparse_set) => sparse_set,
            None => unreachable_unchecked(),
        }
    })
}

unsafe fn downcast_sparse_set_mut<T>(
    sparse_set: AtomicRefMut<dyn AbstractSparseSet>,
) -> AtomicRefMut<SparseSet<T>>
where
    T: Component,
{
    AtomicRefMut::map(sparse_set, |sparse_set| {
        match sparse_set.downcast_mut::<SparseSet<T>>() {
            Some(sparse_set) => sparse_set,
            None => unreachable_unchecked(),
        }
    })
}
