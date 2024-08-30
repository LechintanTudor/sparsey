use crate::component::{Component, ComponentData, GroupDescriptor, GroupLayout};
use crate::world::World;
use alloc::vec::Vec;

/// Builder struct for creating a [`World`].
#[must_use]
#[derive(Clone, Default, Debug)]
pub struct WorldBuilder {
    layout: GroupLayout,
    components: Vec<ComponentData>,
}

impl WorldBuilder {
    /// Sets a group `layout` for the world.
    #[inline]
    pub fn set_layout(&mut self, layout: GroupLayout) -> &mut Self {
        self.layout = layout;
        self
    }

    /// Adds a new component group to the world.
    pub fn add_group<G>(&mut self) -> &mut Self
    where
        G: GroupDescriptor,
    {
        self.add_group_dyn(G::COMPONENTS)
    }

    /// Adds a new component group to the world.
    #[inline]
    pub fn add_group_dyn(&mut self, components: &[ComponentData]) -> &mut Self {
        self.layout.add_group_dyn(components);
        self
    }

    /// Registers a new component type on the world.
    pub fn register<T>(&mut self) -> &mut Self
    where
        T: Component,
    {
        self.register_dyn(ComponentData::new::<T>())
    }

    /// Registers a new component type on the world.
    #[inline]
    pub fn register_dyn(&mut self, component: ComponentData) -> &mut Self {
        self.components.push(component);
        self
    }

    /// Buidls the world with the previously specified options.
    ///
    /// Returns the newly created world.
    #[must_use]
    pub fn build(&self) -> World {
        let mut world = World::new(&self.layout);

        for &component in &self.components {
            world.register_dyn(component);
        }

        world
    }
}
