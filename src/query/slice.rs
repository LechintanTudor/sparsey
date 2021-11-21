use crate::query::{FetchComponent, FetchComponentSet};
use crate::storage::Entity;

pub trait SliceComponent<'a>
where
    Self: FetchComponent<'a>,
{
    fn entities(&self) -> &[Entity];

    fn components(&self) -> &[Self::Component];
}

pub trait SliceComponentSet<'a>
where
    Self: FetchComponentSet<'a>,
{
    type Slices;
}
