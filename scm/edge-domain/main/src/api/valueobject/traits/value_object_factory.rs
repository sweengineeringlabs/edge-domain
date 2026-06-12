//! [`ValueObjectFactory`] — constructor contract for value object types.

use crate::api::valueobject::errors::value_object_error::ValueObjectError;
use crate::api::valueobject::types::non_empty_string::NonEmptyString;

/// Factory trait for the standard value-object implementations.
pub trait ValueObjectFactory {
    /// Construct a [`NonEmptyString`], returning [`ValueObjectError::Empty`] if `s` is empty.
    fn non_empty_string(s: String) -> Result<NonEmptyString, ValueObjectError> {
        NonEmptyString::new(s).map_err(|_| ValueObjectError::Empty)
    }
}
