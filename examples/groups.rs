use ecstasy::prelude::*;
use ecstasy::storage::SparseSet;

fn main() {
    let e0 = Entity::with_id(0);
    let e1 = Entity::with_id(1);
    let e2 = Entity::with_id(2);
    let e3 = Entity::with_id(3);

    let mut set1 = SparseSet::<i32>::default();
    set1.insert(e0, 0);
    set1.insert(e1, 1);
    set1.insert(e2, 2);
    set1.insert(e3, 3);

    let mut set2 = SparseSet::<i32>::default();
    set2.insert(e3, 3);
    set2.insert(e2, 2);
    set2.insert(e1, 1);
    set2.insert(e0, 0);

    let mut set3 = SparseSet::<i32>::default();
    set3.insert(e2, 2);
    set3.insert(e1, 1);
    set3.insert(e0, 0);

    let mut group_len = 0;

    group_len += try_group(&mut [&mut set1, &mut set2, &mut set3], e0, group_len) as usize;
    group_len += try_group(&mut [&mut set1, &mut set2, &mut set3], e1, group_len) as usize;
    group_len += try_group(&mut [&mut set1, &mut set2, &mut set3], e2, group_len) as usize;
    group_len += try_group(&mut [&mut set1, &mut set2, &mut set3], e3, group_len) as usize;

    println!("Group length: {}\n", group_len);
    println!("{:?}\n{:?}\n", set1.dense(), set1.data());
    println!("{:?}\n{:?}\n", set2.dense(), set2.data());
    println!("{:?}\n{:?}\n", set3.dense(), set3.data());

    // group_len -= try_ungroup(&mut [&mut set1, &mut set2, &mut set3], e0, group_len) as usize;
    group_len -= try_ungroup(&mut [&mut set1, &mut set2, &mut set3], e1, group_len) as usize;
    group_len -= try_ungroup(&mut [&mut set1, &mut set2, &mut set3], e2, group_len) as usize;
    group_len -= try_ungroup(&mut [&mut set1, &mut set2, &mut set3], e3, group_len) as usize;

    set1.remove(e1);
    set2.remove(e2);
    set3.remove(e3);

    println!("Group lenth: {}\n", group_len);
    println!("{:?}\n{:?}\n", set1.dense(), set1.data());
    println!("{:?}\n{:?}\n", set2.dense(), set2.data());
    println!("{:?}\n{:?}\n", set3.dense(), set3.data());
}

fn try_group(sets: &mut [&mut SparseSet<i32>], entity: Entity, len: usize) -> bool {
    let mut needs_grouping = false;

    for set in sets.iter() {
        if let Some(index_entity) = set.sparse().get(entity) {
            if index_entity.index() >= len {
                needs_grouping |= true;
            }
        } else {
            return false;
        }
    }

    if needs_grouping {
        for set in sets.iter_mut() {
            swap_elements(set, entity, len);
        }
    }

    needs_grouping
}

fn try_group2(sets: &mut [&mut SparseSet<i32>], entities: &[Entity], len: usize) {
    for set in sets.iter() {}
}

fn try_ungroup(sets: &mut [&mut SparseSet<i32>], entity: Entity, len: usize) -> bool {
    if len == 0 {
        return false;
    }

    let mut needs_ungrouping = false;

    for set in sets.iter() {
        if let Some(index_entity) = set.sparse().get(entity) {
            if index_entity.index() < len {
                needs_ungrouping |= true;
            }
        } else {
            return false;
        }
    }

    if needs_ungrouping {
        let last_index = len - 1;

        for set in sets.iter_mut() {
            swap_elements(set, entity, last_index);
        }
    }

    needs_ungrouping
}

fn swap_elements(set: &mut SparseSet<i32>, entity: Entity, index: usize) {
    let entity_index = set.sparse().get(entity).unwrap().index();
    set.swap(entity_index, index);
}
