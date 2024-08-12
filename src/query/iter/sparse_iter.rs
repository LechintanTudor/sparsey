use crate::entity::Entity;
use crate::query::{Query, WorldQueryAll};
use std::ptr;
use std::slice::Iter as SliceIter;

pub struct SparseIter<'query, 'view, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    entities: SliceIter<'query, Entity>,
    query: &'query mut WorldQueryAll<'view, G, I, E>,
}

impl<'query, 'view, G, I, E> SparseIter<'query, 'view, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub fn new(query: &'query mut WorldQueryAll<'view, G, I, E>) -> Self {
        let entities = {
            let mut entities = G::entities(&query.get);

            if let Some(include_entities) = I::entities(&query.include) {
                match entities {
                    Some(old_entities) => {
                        if include_entities.len() < old_entities.len() {
                            entities = Some(include_entities);
                        }
                    }
                    None => entities = Some(include_entities),
                }
            };

            unsafe { &*ptr::from_ref(entities.unwrap_or(query.world.entities())) }
        };

        Self {
            entities: entities.iter(),
            query,
        }
    }
}

impl<'query, G, I, E> Iterator for SparseIter<'query, '_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = G::Item<'query>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(&entity) = self.entities.next() else {
                break None;
            };

            if !E::contains_none(&self.query.exclude, entity) {
                continue;
            }

            if !I::contains_all(&self.query.include, entity) {
                continue;
            }

            unsafe {
                let Some(item) = G::get(&self.query.get, entity) else {
                    continue;
                };

                break Some(item);
            }
        }
    }
}
