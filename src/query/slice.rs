use crate::query::{GetComponentSetUnfiltered, GetComponentUnfiltered};
use crate::storage::Entity;

pub trait SliceComponent<'a>
where
    Self: GetComponentUnfiltered<'a>,
{
    fn entities(&self) -> &[Entity];

    fn components(&self) -> &[Self::Component];
}

pub trait SliceComponentSet<'a>
where
    Self: GetComponentSetUnfiltered<'a>,
{
    type Slices;
}
