use self::definition::*;
use std::{any::TypeId, cmp::Ordering};

mod definition;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GroupComparissonResult {
    Laxer,
    Equal,
    Stricter,
    Exclusive,
    Overlapped,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Group {
    components: Box<[TypeId]>,
}

impl Group {
    pub fn new<D>() -> Self
    where
        D: GroupDefinition,
    {
        let mut components = D::components();
        components.dedup();
        components.sort_unstable();

        Self {
            components: components.into(),
        }
    }

    pub fn contains(&self, component: &TypeId) -> bool {
        self.components.binary_search(component).is_ok()
    }

    pub fn compare(&self, other: &Group) -> GroupComparissonResult {
        let overlaps = {
            let (g1, g2) = if self.len() <= other.len() {
                (self, other)
            } else {
                (other, self)
            };

            g1.components.iter().fold(0, |overlaps, component| {
                if g2.contains(component) {
                    overlaps + 1
                } else {
                    overlaps
                }
            })
        };

        match self.len().cmp(&other.len()) {
            Ordering::Less => {
                if overlaps == 0 {
                    GroupComparissonResult::Exclusive
                } else if overlaps == self.len() {
                    GroupComparissonResult::Laxer
                } else {
                    GroupComparissonResult::Overlapped
                }
            }
            Ordering::Equal => {
                if overlaps == 0 {
                    GroupComparissonResult::Exclusive
                } else if overlaps == self.len() {
                    GroupComparissonResult::Equal
                } else {
                    GroupComparissonResult::Overlapped
                }
            }
            Ordering::Greater => {
                if overlaps == 0 {
                    GroupComparissonResult::Exclusive
                } else if overlaps == other.len() {
                    GroupComparissonResult::Stricter
                } else {
                    GroupComparissonResult::Overlapped
                }
            }
        }
    }

    pub fn components(&self) -> &[TypeId] {
        &self.components
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    #[test]
    fn compare() {
        let g1 = Group::new::<(A, B, C)>();
        let g2 = Group::new::<(A, B, C, D)>();
        let g3 = Group::new::<(A, B, C)>();
        let g4 = Group::new::<(A, B)>();
        let g5 = Group::new::<(A, D)>();
        let g6 = Group::new::<(D, E)>();

        assert_eq!(g1.compare(&g2), GroupComparissonResult::Laxer);
        assert_eq!(g1.compare(&g3), GroupComparissonResult::Equal);
        assert_eq!(g1.compare(&g4), GroupComparissonResult::Stricter);
        assert_eq!(g1.compare(&g5), GroupComparissonResult::Overlapped);
        assert_eq!(g1.compare(&g6), GroupComparissonResult::Exclusive);
    }
}
