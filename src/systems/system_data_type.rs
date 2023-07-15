use crate::utils::{ComponentData, ResourceData};

/// Type of asset borrowed by systems during execution.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SystemDataType {
    /// View over all entities.
    Entities,
    /// View over all components of a type.
    Comp(ComponentData),
    /// Mutable view over all components of a type.
    CompMut(ComponentData),
    /// View over a resource.
    Res(ResourceData),
    /// Mutable view over a resource.
    ResMut(ResourceData),
}

impl SystemDataType {
    /// Returns `true` if the parameters prevent the systems from running in parallel.
    #[must_use]
    pub fn conflicts_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Comp(c1), Self::CompMut(c2)) => c1 == c2,
            (Self::CompMut(c1), Self::Comp(c2)) => c1 == c2,
            (Self::CompMut(c1), Self::CompMut(c2)) => c1 == c2,
            (Self::Res(r1), Self::ResMut(r2)) => r1 == r2,
            (Self::ResMut(r1), Self::Res(r2)) => r1 == r2,
            (Self::ResMut(r1), Self::ResMut(r2)) => r1 == r2,
            _ => false,
        }
    }
}
