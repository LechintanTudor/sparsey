use crate::components::{Component, ComponentGroupInfo};
use crate::query::{ChangeTicksFilter, ComponentViewData};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::Ticks;

/// Trait used to fetch a component from a component view.
pub unsafe trait GetComponentUnfiltered<'a> {
    /// Fetched item.
    type Item: 'a;
    /// Component type of fetched item.
    type Component: Component;

    /// Returns the group to which the storage belongs, if any.
    fn group_info(&self) -> Option<ComponentGroupInfo<'a>>;

    /// Returns the world tick and change tick used for change detection.
    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    /// Returns the dense index mapped to `entity`.
    fn get_index(&self, entity: Entity) -> Option<usize>;

    /// Returns `true` if the data at the given index matches the filter.
    unsafe fn matches_unchecked<F>(&self, index: usize) -> bool
    where
        F: ChangeTicksFilter;

    /// Returns the item at the given index and whether or not it matches the filter.
    unsafe fn get_unchecked<F>(self, index: usize) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter;

    /// Splits the view into its parts.
    fn split(self) -> (&'a [Entity], &'a EntitySparseArray, ComponentViewData<Self::Component>);

    /// Returns the item at the given index and whether or not it matches the filter.
    unsafe fn get_from_parts_unchecked<F>(
        data: ComponentViewData<Self::Component>,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter;
}

/// Trait used to slice queries.
pub unsafe trait GetImmutableComponentUnfiltered<'a>: GetComponentUnfiltered<'a> {
    /// Returns all entities in the storage as a slice.
    fn entities(&self) -> &'a [Entity];

    /// Returns all component in the storage as a slice.
    fn components(&self) -> &'a [Self::Component];
}
