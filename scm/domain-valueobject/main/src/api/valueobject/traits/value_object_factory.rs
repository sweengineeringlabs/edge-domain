//! `ValueObjectFactory` — factory trait for value object construction.

use crate::api::valueobject::errors::value_object_error::ValueObjectError;
use crate::api::valueobject::types::non_empty_string::NonEmptyString;

/// Factory for constructing value objects with default implementations.
pub trait ValueObjectFactory {
    /// Construct a [`NonEmptyString`] from the given string, returning
    /// [`ValueObjectError::Empty`] if the string is empty.
    fn non_empty_string(s: String) -> Result<NonEmptyString, ValueObjectError> {
        NonEmptyString::new(s).map_err(|_| ValueObjectError::Empty)
    }
}
