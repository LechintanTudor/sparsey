use crate::Component;
use std::{any::TypeId, cmp::Ordering, collections::BTreeSet};

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Group {
    type_ids: Box<[TypeId]>,
}

impl Group {
    pub fn builder() -> GroupBuilder {
        Default::default()
    }

    pub fn contains<T>(&self) -> bool
    where
        T: Component,
    {
        self.contains_type_id(TypeId::of::<T>())
    }

    pub fn contains_type_id(&self, type_id: TypeId) -> bool {
        self.type_ids.binary_search(&type_id).is_ok()
    }

    pub fn type_ids(&self) -> &[TypeId] {
        &self.type_ids
    }

    pub fn len(&self) -> usize {
        self.type_ids.len()
    }
}

impl PartialOrd for Group {
    /// A group is considered greater than other group if it is less restrictive.
    /// If neither group is contained withing the other, the groups are uncomparable.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.len().cmp(&other.len()) {
            Ordering::Equal => {
                if self.type_ids == other.type_ids {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            Ordering::Less => {
                for &type_id in self.type_ids.iter() {
                    if !other.contains_type_id(type_id) {
                        return None;
                    }
                }

                Some(Ordering::Greater)
            }
            Ordering::Greater => {
                for &type_id in other.type_ids.iter() {
                    if !self.contains_type_id(type_id) {
                        return None;
                    }
                }

                Some(Ordering::Less)
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GroupBuilder {
    type_ids: BTreeSet<TypeId>,
}

impl GroupBuilder {
    pub fn add<T>(&mut self)
    where
        T: Component,
    {
        self.type_ids.insert(TypeId::of::<T>());
    }

    pub fn type_id(&mut self, type_id: TypeId) {
        self.type_ids.insert(type_id);
    }

    pub fn with<T>(mut self) -> Self
    where
        T: Component,
    {
        self.add::<T>();
        self
    }

    pub fn with_type_id(mut self, type_id: TypeId) -> Self {
        self.type_id(type_id);
        self
    }

    pub fn build(self) -> Group {
        Group {
            type_ids: self.type_ids.into_iter().collect::<_>(),
        }
    }
}
