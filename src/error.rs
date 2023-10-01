//! Implements the crate's error type

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    ffi::NulError,
    fmt::{self, Display, Formatter},
    io,
};

/// Creates a new error
#[macro_export]
macro_rules! error {
    (with: $source:expr, $($arg:tt)*) => {{
        let error = format!($($arg)*);
        $crate::error::Error::with(error, $source)
    }};
    ($($arg:tt)*) => {{
        let error = format!($($arg)*);
        $crate::error::Error::new(error)
    }};
}

/// The crates error type
#[derive(Debug)]
pub struct Error {
    /// The error description
    error: String,
    /// The underlying error
    source: Option<Box<dyn std::error::Error + Send>>,
    /// The backtrace
    backtrace: Backtrace,
}
impl Error {
    /// Creates a new error
    pub fn new<T>(error: T) -> Self
    where
        T: ToString,
    {
        let backtrace = Backtrace::capture();
        Self { error: error.to_string(), source: None, backtrace }
    }
    /// Creates a new error
    pub fn with<T, S>(error: T, source: S) -> Self
    where
        T: ToString,
        S: std::error::Error + Send + 'static,
    {
        let source = Box::new(source);
        let backtrace = Backtrace::capture();
        Self { error: error.to_string(), source: Some(source), backtrace }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Print the error
        writeln!(f, "{}", self.error)?;

        // Print backtrace
        if self.backtrace.status() == BacktraceStatus::Captured {
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            writeln!(f, "{}", self.backtrace)?;
        }
        Ok(())
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let source = self.source.as_ref()?;
        Some(source.as_ref())
    }
}
impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        let error = source.to_string();
        Self::with(error, source)
    }
}
impl From<NulError> for Error {
    fn from(source: NulError) -> Self {
        let error = source.to_string();
        Self::with(error, source)
    }
}
impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        let error = source.to_string();
        Self::with(error, source)
    }
}
