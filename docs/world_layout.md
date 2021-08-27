# World Layout
How a `Layout` affects components within a `World` \~( ˘▾˘\~)

## Component Storage
Sparsey's component storages are based on a data structure called [sparse set](https://skypjack.github.io/2020-08-02-ecs-baf-part-9/). The advantage of this form of component storage is that the entities and components are stored in packed arrays which give us the best performance when iterating all components in a storage and allow us to get a slice with all components of a particular type.

## Groups
Groups allow us to greatly optimize iteration speed of some queries by splitting the component storages into two halves: one which contains all components of entities which also have components in the other storages, and the other of entities which do not have the components from other storages. Basically, if we group the `A` and `B` storges:
- Before grouping.
```
Storage A: (Components in random order)
[------------------------------]

Storage B: (Components in random order)
[------------------------------]
```

- After grouping
```
Storage A:
       1)              2)
[--------------][--------------]
1) Components of entities which have B
2) Components of entities which do not have B

Storage B:
       1)              2)
[--------------][--------------]
1) Components of entities which have A
2) Components of entities which do not have A
```

Because the components which belong to entities which have both `A` and `B` are stored at the beginning of their storages, iterating a query which fetches the components of entities with `A` and `B` is much faster than for ungrouped storages.
```rust
for (a, b) in (&a, &b).iter() {
    // ...
}
```

## Nested Groups
