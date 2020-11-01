pub use {code_gen::*, view::*};

mod code_gen;
mod view;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, SparseSet};

    #[test]
    fn iterator2() {
        let mut s1 = SparseSet::<u32>::default();
        s1.insert(Entity::new(0, 0), 0);
        s1.insert(Entity::new(1, 0), 1);
        s1.insert(Entity::new(2, 0), 2);

        let mut s2 = SparseSet::<i32>::default();
        s2.insert(Entity::new(3, 0), 3);
        s2.insert(Entity::new(2, 0), 2);
        s2.insert(Entity::new(1, 0), 1);
        s2.insert(Entity::new(0, 0), 0);

        let mut iterator = Iterator2::<&u32, Option<&mut i32>>::new(&s1, &mut s2);
        assert!(matches!(iterator.next(), Some((&0, Some(&mut 0)))));
        assert!(matches!(iterator.next(), Some((&1, Some(&mut 1)))));
        assert!(matches!(iterator.next(), Some((&2, Some(&mut 2)))));
    }
}
