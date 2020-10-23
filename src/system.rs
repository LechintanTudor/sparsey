use crate::World;

pub struct SubWorld<'a> {
    world: &'a World,
}

impl SubWorld<'_> {
    pub fn iter(&self) {}

    pub fn iter_mut(&mut self) {}
}
