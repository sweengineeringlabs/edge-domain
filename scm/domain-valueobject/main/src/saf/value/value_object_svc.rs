//! SAF — [`ValueObject`], [`NonEmptyString`], and [`ValueObjectError`] re-exports.

pub use crate::api::ValueObjectError;
pub use crate::api::ValueObject;
pub use crate::api::NonEmptyString;

/// Identifies the value-object SAF contract in this crate.
pub const VALUE_OBJECT_SVC: &str = "value_object";
