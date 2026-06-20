//! SAF — [`ValueObjectBootstrap`] re-export.
//!
//! Value-object construction is exposed as methods on the bootstrap trait
//! (`<T as ValueObjectBootstrap>::non_empty_string`), not as free functions —
//! see SEA rule 191. `NonEmptyString` implements `ValueObjectBootstrap`, so
//! callers construct via `NonEmptyString::non_empty_string(..)`.

pub use crate::api::ValueObjectBootstrap;

/// Identifies the value-object-factory SAF contract in this crate.
pub const VALUE_OBJECT_FACTORY_SVC: &str = "value_object_factory";
