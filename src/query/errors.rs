use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct StoragesNotGrouped;

impl Error for StoragesNotGrouped {}

impl Display for StoragesNotGrouped {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Component storages not grouped")
	}
}
