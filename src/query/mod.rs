//! Fetch and iterate components and entities that match a query.

pub use self::component_view::*;
pub use self::compound_query::*;
pub use self::iter::*;
pub use self::query::*;
pub use self::query_group_info::*;

mod component_view;
mod compound_query;
mod iter;
mod query;
mod query_group_info;
