use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

pub type SystemError = anyhow::Error;
pub type RunResult = Result<(), RunError>;
pub type SystemResult = Result<(), SystemError>;

pub struct RunError {
    sources: Vec<SystemError>,
}

impl RunError {
    pub(crate) fn new(sources: Vec<SystemError>) -> Self {
        Self { sources }
    }

    pub fn errors(&self) -> impl Iterator<Item = &(dyn Error + Send + Sync + 'static)> {
        self.sources.iter().map(|e| e.deref())
    }

    pub fn into_errors(self) -> impl Iterator<Item = SystemError> {
        self.sources.into_iter()
    }
}

impl Error for RunError {}

impl Debug for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.sources.first().unwrap(), f)
    }
}

impl Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.sources.first().unwrap(), f)
    }
}
