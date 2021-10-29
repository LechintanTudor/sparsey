use crate::components::Component;
use crate::query::QueryElementFilter;
use crate::storage::{ComponentStorageData, Entity, EntitySparseArray, IndexEntity};

pub trait QueryElementBase<'a> {
    type Item: 'a;
    type Component: Component;

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity>;

    fn contains<F>(&self, entity: Entity, filter: &F) -> bool
    where
        F: QueryElementFilter<Self::Component>;

    unsafe fn get_unchecked<F>(self, index: usize, filter: &F) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>;

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        &'a ComponentStorageData,
    );

    unsafe fn get_from_parts_unchecked<F>(
        data: &ComponentStorageData,
        index: usize,
        filter: &F,
    ) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>;
}
