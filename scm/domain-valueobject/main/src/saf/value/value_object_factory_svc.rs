//! SAF — [`ValueObjectFactory`] re-export.
//!
//! Value-object construction is exposed as methods on the factory trait
//! (`<T as ValueObjectFactory>::non_empty_string`), not as free functions —
//! see SEA rule 191. `NonEmptyString` implements `ValueObjectFactory`, so
//! callers construct via `NonEmptyString::non_empty_string(..)`.

pub use crate::api::valueobject::traits::value_object_factory::ValueObjectFactory;

/// Identifies the value-object-factory SAF contract in this crate.
pub const VALUE_OBJECT_FACTORY_SVC: &str = "value_object_factory";
