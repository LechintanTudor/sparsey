use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// Type erased error returned by systems.
pub type SystemError = anyhow::Error;
/// Result returned by `Dispatcher::run_seq` and `Dispatcher::run_par`.
pub type RunResult = Result<(), RunError>;
/// Result returned by systems.
pub type SystemResult = anyhow::Result<()>;

/// Error returned by `Dispatcher::run_seq` and `Dispatcher::run_par`.
pub struct RunError {
    errors: Vec<SystemError>,
}

impl RunError {
    /// Returns all `SystemError`s as a slice.
    pub fn errors(&self) -> &[SystemError] {
        &self.errors
    }

    /// Returns all `SystemError`s as a vector.
    pub fn into_errors(self) -> Vec<SystemError> {
        self.errors
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

impl Deref for RunError {
    type Target = [SystemError];

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.errors.first().map(|error| error.as_ref())
    }
}

impl Debug for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.errors.first() {
            Some(error) => Debug::fmt(error, f),
            None => Ok(()),
        }
    }
}

impl Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.errors.first() {
            Some(error) => Display::fmt(error, f),
            None => Ok(()),
        }
    }
}
