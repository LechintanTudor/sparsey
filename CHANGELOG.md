# 0.12.1 (2024-06-29)

## Changed

- Updated `rustc-hash` to `v2.0.0`.

# 0.12.0 (2024-01-27)

Complete rewrite improving performance in all benchmarks.

## Changed

- `World` renamed to `EntityStorage`.
- `Resources` renamed to `ResourceStorage`.
- `World` now includes an `EntityStorage` and a `ResourceStorage`.

## Removed

- Removed `Schedule` and `ScheduleBuilder`.
- Resources can no longer be `!Send` or `!Sync`.

# 0.11.1 (2023-09-27)

## Added

- Added `get_mut` and `try_get_mut` functions for getting mutable references to
  resources from a `&mut Resources`.

# 0.11.0 (2023-07-16)

## Added

- Added functions for running systems that only borrow data from `World`,
  `Resources` or `SyncResources`.
- Added `Debug` implementations for most public data types.

## Changed

- Refactored `ComponentStorage` and `ComponentStorages` to significantly improve
  the performance of all component-related operations.

# 0.10.0 (2022-11-22)

## Added

- Added `run`, `run_local` and `run_exclusive` to provide a uniform interface
  for running systems and functions that borrow assets from `World` and
  `Resources`.

## Changed

- Updated the `query` and `systems` modules to take advantage of GATs.
- `EntityStorage` is no longer paginated, improving performance for all
  operations.

# 0.9.0 (2022-07-23)

## Changed

- Entities are now ordered by their version first instead of their id, so
  entities created earlier show up first when sorting.
- Various bitmasks used internally are now wrapped in new types to make the code
  easier to maintain.
- Renamed `Query` to `QueryPart` and `CompoundQuery` to `Query`.

## Fixed

- `CompMut::get_mut` now takes `self` by mutable reference instead of shared
  reference.

# 0.8.1 (2022-07-09)

## Changed

- `Schedule` now calls `World::maintain` in a more predictable manner.
- Entities are now recycled in the order in which they were deallocated.

## Fixed

- Fixed panic in `EntityAllocator` when calling `maintain` multiple times.
- `EntityAllocator::clear` no longer prevents unmainted entities from being
  recycled.
- `EntityAllocator` now correctly allocates entities with ids up to `u32::MAX`
  instead of `u32::MAX - 1`.

# 0.8.0 (2022-05-28)

## Added

- Added panicking functions for borrowing resources.
- Added `World::reset` to reset the world's entity allocator.

## Fixed

- Fixed bug that prevented `ComponentStorage` from swapping components.

# 0.7.0 (2022-02-18)

## Added

- Added `for_each` and `for_each_entity` methods to queries.
- Adding a system a `Schedule` no longer requires calling `.system()` on the
  function.

## Changed

- Removed resources from the `World`. Use `Resources` to create and manage
  resources.
- Improved the system scheduling algorithm to allow more systems to run in
  parallel.
- Renamed `Dispatcher` to `Schedule`.
- Renamed query slicing methods to `as_entity_slice`, `as_component_slices`,
  `as_entity_and_component_slices`.
- Refactored `ComponentStorage` to optimize component swapping.

## Removed

- Removed `ChangeTicks` and component change detection.
- Removed `Commands`. Use `Entities` to create entities inside systems.

## Fixed

- Invalid `Layout`s can no longer be created.

# 0.6.0 (2021-12-20)

## Added

- Allow using change detection on multiple `ComponentView`s at the same time.

## Changed

- Moved items from crate root into separate modules.
- Renamed `World::append_components` to `World::insert_components`.
- `World::incrment_tick` now panics instead of returning an error on overflow.

## Fixed

- Fixed `ComponentStorage::clear` not removing the entities from its
  `SparseArray`.
- `World` keeps generating unique entities after calling
  `World::clear_entities`.
- Filtered queries are no longer sliceable.

# 0.5.0 (2021-11-09)

## Added

- Implemented `fold` for `EntityIter`, greatly improving the performance of
  `for_each`.
- Implemented `fmt::Debug` for `ComponentView` and `ResourceView`.
- Added `World::delete_resource` to remove a `Resource` with a given `TypeId`.

## Changed

- Cleaned up `QueryElement` to make the code faster and easier to maintain.
- Replaced `ComponentView::storage` with methods on `ComponentView`.
- `World::increment_tick` no longer sets the world tick to zero on overflow.

## Fixed

- `IntoEntityIterator` is now only implemented for `EntityIterator`s, not all
  `Iterator`s.

# 0.4.0 (2021-10-17)

## Added

- `World::borrow` now accepts `Option<Res<T>>` and `Option<ResMut<T>>`.
- Systems can now have `Option<Res<T>>` and `Option<ResMut<T>>` as parameters.
- Added `ComponentView::storage` to get a reference to the view's
  `ComponentStorage`.
- Added `#[must_use]` to functions whose returns should not be discarded.

## Changed

- Reworked `ComponentStorage` to make adding, removing and swapping components
  faster.
- Optimize the implementation of `ComponentSet` for the unit type.
- All methods of `ComponentSet` are now safe.
- Reduce size of `System`, `LocalSystem` and `GroupInfo` structs.
- Items not meant to be used outside `Sparsey` are now `pub(crate)`.

## Removed

- Removed `World::resources` and `World::storages` iterators.

## Fixed

- Removed debug `println` from `ComponentStorages`.

# 0.3.1 (2021-10-04)

## Changed

- Inlined some functions to improve iteration performance.

# 0.3.0 (2021-09-28)

## Added

- Added `World::resources` for iterating over all `Resource`s and their
  `TypeId`s.
- Added `World::storages` for iterating over all `ComponentStorage`s and the
  `TypeId`s of the components they hold.

## Changed

- Refactor `BlobVec`, improving the performance of adding, removing and updating
  components.
- Improved performance of `World::create_entities` when the components belong to
  groups.
- Simplify `QueryModifier` to improve the performance of creating iterators.
- Changed visibility of `TypedComponentStorage` to public.
- Improved performance of grouping and ungrouping components.

## Removed

- Removed all methods from `ComponentView`.

## Fixed

- Removing a component from a nested group no longer ungroups the components of
  the parent groups.

# 0.2.0 (2021-09-19)

## Added

- Queries over a single component view no longer need to be wrapped in a tuple.
- Added `World::destroy_entities` for destroying entities in bulk.
- Added `World::set_tick` for setting the tick used in change detection.

## Changed

- Iterators over a single component view are now dense, greatly improving
  performance.
- Refactor `ComponentSet`, improving the performance of adding and removing
  components.

## Fixed

- Fixed `!added`, `!mutated`, `!changed` not being usable in queries.

# 0.1.0 (2021-09-12)

- First version.
