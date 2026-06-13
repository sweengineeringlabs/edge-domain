//! SAF — [`ValueObjectFactory`] re-export and [`non_empty_string`] factory function.

pub use crate::api::valueobject::traits::value_object_factory::ValueObjectFactory;

use crate::api::valueobject::errors::ValueObjectError;
use crate::api::valueobject::types::NonEmptyString;

/// Construct a [`NonEmptyString`], returning [`ValueObjectError::Empty`] when `s` is empty.
pub fn non_empty_string(s: String) -> Result<NonEmptyString, ValueObjectError> {
    NonEmptyString::new(s).map_err(|_| ValueObjectError::Empty)
}
