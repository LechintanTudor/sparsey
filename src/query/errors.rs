use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Error returned when trying to slice a query with ungrouped component
/// storages.
#[derive(Debug)]
pub struct UngroupedComponentStorages;

impl Error for UngroupedComponentStorages {}

impl Display for UngroupedComponentStorages {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tried to slice query with ungrouped component storages")
	}
}
