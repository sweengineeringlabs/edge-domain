//! `ValueObject` marker trait.

use std::hash::Hash;

use crate::api::valueobject::errors::ValueObjectError;
use crate::api::valueobject::types::ValidationRequest;

/// Marker trait for value objects.
///
/// A value object has no identity of its own — two instances are equal when
/// all their fields are equal.  Implementors must be equality-comparable,
/// hashable, cloneable, and safe to share across threads.
///
/// Concrete value objects belong in the `api/<theme>/types/` directory of the
/// theme that owns them.  `NonEmptyString` ships as a reference implementation
/// in this crate only.
pub trait ValueObject: Eq + Hash + Clone + Send + Sync {
    /// Re-check this value object's invariants.
    ///
    /// The default implementation always succeeds — override it when a
    /// value object can become invalid through a construction path other
    /// than its normal constructor (e.g. deserialization).
    fn validate(&self, _req: ValidationRequest) -> Result<(), ValueObjectError> {
        Ok(())
    }
}
