use std::error::Error;
use std::fmt;

/// Error returned when trying to insert components to entities not present in a world.
#[derive(Debug)]
pub struct NoSuchEntity;

impl Error for NoSuchEntity {}

impl fmt::Display for NoSuchEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No such entity was found in the World")
    }
}
