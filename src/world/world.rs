use crate::data::{AtomicRef, AtomicRefMut, Component, Entity, TypeErasedSparseSet};
use crate::world::{
    Comp, CompMut, Entities, GroupedComponents, UngroupedComponents, WorldLayoutDescriptor,
};
use std::collections::HashSet;
use std::iter;
use std::{any::TypeId, borrow::Borrow};

pub struct World {
    entities: Entities,
    grouped_components: GroupedComponents,
    ungrouped_components: UngroupedComponents,
    group_indexes: HashSet<usize>,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Entities::default(),
            grouped_components: GroupedComponents::new(&L::world_layout()),
            ungrouped_components: UngroupedComponents::default(),
            group_indexes: HashSet::default(),
        }
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        if !self.grouped_components.contains(&TypeId::of::<T>()) {
            self.ungrouped_components.register::<T>();
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

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        match self.grouped_components.borrow(&TypeId::of::<T>()) {
            Some(sparse_set) => unsafe {
                Some(Comp::new(
                    AtomicRef::map_into(sparse_set, |sparse_set| sparse_set.to_ref()),
                    self.grouped_components.get_group_len(&TypeId::of::<T>()),
                ))
            },
            None => match self.ungrouped_components.borrow(&TypeId::of::<T>()) {
                Some(sparse_set) => unsafe {
                    Some(Comp::new(
                        AtomicRef::map_into(sparse_set, |sparse_set| sparse_set.to_ref()),
                        None,
                    ))
                },
                None => None,
            },
        }
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        match unsafe { self.grouped_components.borrow_mut(&TypeId::of::<T>()) } {
            Some(sparse_set) => unsafe {
                Some(CompMut::new(
                    AtomicRefMut::map_into(sparse_set, |sparse_set| sparse_set.to_ref_mut()),
                    self.grouped_components.get_group_len(&TypeId::of::<T>()),
                ))
            },
            None => match self.ungrouped_components.borrow_mut(&TypeId::of::<T>()) {
                Some(sparse_set) => unsafe {
                    Some(CompMut::new(
                        AtomicRefMut::map_into(sparse_set, |sparse_set| sparse_set.to_ref_mut()),
                        None,
                    ))
                },
                None => None,
            },
        }
    }

    // pub fn create<C>(&mut self, components: C) -> Entity
    // where
    //     C: ComponentSet,
    // {
    //     let entity = self.entities.create();
    //     self.append(entity, components).unwrap();
    //     entity
    // }

    // pub fn destroy(&mut self, entity: Entity) -> bool {
    //     if !self.entities.destroy(entity) {
    //         return false;
    //     }

    //     unsafe {
    //         for i in 0..self.components.group_count() {
    //             self.components.ungroup_components(i, iter::once(entity));
    //         }

    //         for sparse_set in self.components.iter_sparse_sets_mut() {
    //             sparse_set.delete(entity);
    //         }
    //     }

    //     true
    // }

    // pub fn append<C>(&mut self, entity: Entity, components: C) -> Result<(), ()>
    // where
    //     C: ComponentSet,
    // {
    //     if !self.entities.contains(entity) {
    //         return Err(());
    //     }

    //     unsafe {
    //         {
    //             let mut sparse_set_set = <C::Borrow as BorrowSparseSetSet>::borrow(self);
    //             C::append(&mut sparse_set_set, entity, components);
    //         }

    //         self.update_group_indexes(C::components());

    //         for &i in self.group_indexes.iter() {
    //             self.components.group_components(i, iter::once(entity));
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
    // where
    //     C: ComponentSet,
    // {
    //     if !self.entities.contains(entity) {
    //         return None;
    //     }

    //     unsafe {
    //         self.update_group_indexes(C::components());

    //         for &i in self.group_indexes.iter() {
    //             self.components.ungroup_components(i, iter::once(entity));
    //         }

    //         let mut sparse_set_set = <C::Borrow as BorrowSparseSetSet>::borrow(self);
    //         C::remove(&mut sparse_set_set, entity)
    //     }
    // }

    // pub fn delete<C>(&mut self, entity: Entity)
    // where
    //     C: ComponentSet,
    // {
    //     if !self.entities.contains(entity) {
    //         return;
    //     }

    //     unsafe {
    //         self.update_group_indexes(C::components());

    //         for &i in self.group_indexes.iter() {
    //             self.components.ungroup_components(i, iter::once(entity));
    //         }

    //         let mut sparse_set_set = <C::Borrow as BorrowSparseSetSet>::borrow(self);
    //         C::delete(&mut sparse_set_set, entity)
    //     }
    // }

    // pub fn clear(&mut self) {
    //     self.entities.clear();
    //     self.components.clear();
    // }

    // pub(crate) unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<SparseSetRefMut<T>>
    // where
    //     T: Component,
    // {
    //     self.components.borrow_sparse_set_mut::<T>()
    // }

    // pub(crate) fn entities(&self) -> &Entities {
    //     &self.entities
    // }

    // fn update_group_indexes<T>(&mut self, type_ids: T)
    // where
    //     T: AsRef<[TypeId]>,
    // {
    //     let components = &self.components;

    //     self.group_indexes.clear();
    //     self.group_indexes.extend(
    //         type_ids
    //             .as_ref()
    //             .iter()
    //             .flat_map(|type_id| components.get_group_index(type_id)),
    //     );
    // }
}
