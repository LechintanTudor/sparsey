use std::any::TypeId;

use crate::components::ComponentStorages;
use crate::resources::{Resource, SyncResources};
use crate::storage::{Component, EntityStorage};
use crate::world::{BorrowSyncWorld, Comp, CompMut, Entities, Res, ResMut};

#[derive(Clone, Copy)]
pub struct SyncWorld<'a> {
    entities: &'a EntityStorage,
    components: &'a ComponentStorages,
    resources: SyncResources<'a>,
}

impl<'a> SyncWorld<'a> {
    pub(crate) fn new(
        entities: &'a EntityStorage,
        components: &'a ComponentStorages,
        resources: SyncResources<'a>,
    ) -> Self {
        Self { entities, components, resources }
    }

    pub fn borrow<T>(&'a self) -> T::Item
    where
        T: BorrowSyncWorld<'a>,
    {
        <T as BorrowSyncWorld>::borrow(self)
    }

    pub(crate) fn borrow_entities(&self) -> Entities {
        Entities::new(&self.entities)
    }

    pub(crate) fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        self.components
            .borrow_with_info(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { Comp::new(storage, info) })
    }

    pub(crate) fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        self.components
            .borrow_with_info_mut(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { CompMut::new(storage, info) })
    }

    pub(crate) fn borrow_res<T>(&self) -> Option<Res<T>>
    where
        T: Resource + Sync,
    {
        self.resources.borrow::<T>().map(Res::new)
    }

    pub(crate) fn borrow_res_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource + Send,
    {
        self.resources.borrow_mut::<T>().map(ResMut::new)
    }
}
