//! Fetch and iterate components and entities that match a query.

pub use self::component_view::*;
pub use self::iter::*;
pub use self::query::*;
pub use self::query_group_info::*;
pub use self::query_part::*;

mod component_view;
mod iter;
mod query;
mod query_group_info;
mod query_part;
