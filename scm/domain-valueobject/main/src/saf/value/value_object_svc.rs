//! SAF — [`ValueObject`], [`NonEmptyString`], and [`ValueObjectError`] re-exports.

pub use crate::api::valueobject::errors::ValueObjectError;
pub use crate::api::valueobject::traits::ValueObject;
pub use crate::api::valueobject::types::NonEmptyString;

/// Identifies the value-object SAF contract in this crate.
pub const VALUE_OBJECT_SVC: &str = "value_object";
