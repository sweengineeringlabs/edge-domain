//! [`ValueObjectError`] — error variants for value-object construction.

use std::fmt;

/// Errors returned when constructing a value object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueObjectError {
    /// The supplied value was empty or blank.
    Empty,
    /// The supplied value failed a format constraint.
    Invalid(String),
}

impl fmt::Display for ValueObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "value must not be empty"),
            Self::Invalid(msg) => write!(f, "invalid value: {msg}"),
        }
    }
}

impl std::error::Error for ValueObjectError {}
