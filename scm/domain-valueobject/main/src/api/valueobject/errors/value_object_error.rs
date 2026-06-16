//! `ValueObjectError` — errors that value object construction can produce.

use std::fmt;

/// Errors returned when constructing a value object fails.
///
/// The canonical case is an invalid input (e.g., an empty string passed to
/// [`NonEmptyString::new`](crate::NonEmptyString::new)).
#[non_exhaustive]
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
