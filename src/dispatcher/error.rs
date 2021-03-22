use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// Type erased error returned by systems.
pub type SystemError = anyhow::Error;

/// Result returned by `Dispatcher::run`.
pub type RunResult = Result<(), RunError>;

/// Result returned by systems.
pub type SystemResult = anyhow::Result<()>;

/// Error returned by `Dispatcher::run`.
pub struct RunError {
    errors: Vec<SystemError>,
}

impl RunError {
    /// Get an iterator over all errors.
    pub fn errors(&self) -> impl Iterator<Item = &(dyn Error + Send + Sync + 'static)> {
        self.errors.iter().map(|e| e.deref())
    }

    /// Get an owning iterator over all errors.
    pub fn into_errors(self) -> impl Iterator<Item = SystemError> {
        self.errors.into_iter()
    }
}

impl From<SystemError> for RunError {
    fn from(error: SystemError) -> Self {
        Self {
            errors: vec![error],
        }
    }
}

impl From<Vec<SystemError>> for RunError {
    fn from(errors: Vec<SystemError>) -> Self {
        Self { errors }
    }
}

impl Error for RunError {}

impl Debug for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.errors.first().unwrap(), f)
    }
}

impl Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.errors.first().unwrap(), f)
    }
}
