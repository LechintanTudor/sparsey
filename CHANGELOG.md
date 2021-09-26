# Unreleased
## Added
- Added `World::resources` for iterating over all `Resource`s and their `TypeId`s.

## Changed
- Refactor `BlobVec`, improving the performance of adding, removing and updating components.
- Improved performance of `World::create_entities` when the components belong to groups.
- Simplify `QueryModifier` to improve the performance of creating iterators.

# 0.2.0 (2021-09-19)
## Added
- Queries over a single component view no longer need to be wrapped in a tuple.
- Added `World::destroy_entities` for destroying entities in bulk.
- Added `World::set_tick` for setting the tick used in change detection.

## Changed
- Iterators over a single component view are now dense, greatly improving performance.
- Refactor `ComponentSet`, improving the performance of adding and removing components.

## Fixed
- Fixed `!added`, `!mutated`, `!changed` not being usable in queries.

# 0.1.0 (2021-09-12)
- First version.
