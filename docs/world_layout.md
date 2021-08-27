# World Layout
How a `Layout` affects components within a `World` \~( ˘▾˘\~)

## Component Storage
Sparsey's component storages are based on a data structure called [sparse set](https://skypjack.github.io/2020-08-02-ecs-baf-part-9/). The advantage of this form of component storage is that the entities and components are stored in packed arrays which give us the best performance when iterating all components in a storage and allow us to get a slice with all components of a particular type.

## Groups
Groups allow us to greatly optimize iteration speed of some queries by splitting the component storages into two halves: one which contains all components of entities which also have components in the other storages, and the other of entities which do not have the components from other storages. Basically, if we group the `A` and `B` storages:
Before grouping.
```
Storage A: (Components in random order)
[------------------------------]

Storage B: (Components in random order)
[------------------------------]
```

After grouping
```
Storage A:
       1)              2)
[--------------][--------------]
1) Components of entities with A and B 
2) Components of entities with A, without B

Storage B:
       1)              2)
[--------------][--------------]
1) Components of entities with A and B 
2) Components of entities with B, without A 
```

Because the components which belong to entities which have both `A` and `B` are stored at the beginning of their storages, iterating a query which fetches the components of entities with `A` and `B` is much faster than for ungrouped storages.
```rust
for (a, b) in (&a, &b).iter() {
    // ...
}
```

## Nested Groups
Nested groups can be created by adding additional storages to an existing group. This further divides the first half of each storage into components of entities which contain all the new components, and the rest. Continuing from the previous example, if we also group the `A`, `B` and `C` storages:
```
Storage A:
     1)         2)         3)
[---------][---------][---------]
1) Components of entities with A, B, and C
2) Components of entities with A and B, without C
3) Components of entities with A, without B 

Storage B:
     1)         2)         3)
[---------][---------][---------]
1) Components of entities with A, B, and C
2) Components of entities with A and B, without C
3) Components of entities with B, without A

Storage C:
     1)              2)    
[---------][--------------------]
1) Components of entities with A, B, and C
2) Components of entities with C, without A or B
```

