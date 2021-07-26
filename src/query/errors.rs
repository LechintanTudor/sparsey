use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Error returned when trying to slice a query for with ungrouped storages.
#[derive(Debug)]
pub struct StoragesNotGrouped;

impl Error for StoragesNotGrouped {}

impl Display for StoragesNotGrouped {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Component storages not grouped")
	}
}
