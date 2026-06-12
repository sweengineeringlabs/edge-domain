//! `ValueObject` marker trait.

use std::hash::Hash;

/// Marker trait for value objects.
///
/// A value object has no identity of its own — two instances are equal when
/// all their fields are equal.  Implementors must be equality-comparable,
/// hashable, cloneable, and safe to share across threads.
///
/// Concrete value objects belong in the `api/<theme>/types/` directory of the
/// theme that owns them.  `NonEmptyString` ships as a reference implementation
/// in this crate only.
pub trait ValueObject: Eq + Hash + Clone + Send + Sync {}
