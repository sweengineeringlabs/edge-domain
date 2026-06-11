//! `ValueObject` marker trait.

use std::hash::Hash;

/// Marker trait for value objects.
///
/// A value object has no identity of its own — two instances are equal when
/// all their fields are equal.  Implementors must be equality-comparable,
/// hashable, cloneable, and safe to share across threads.
///
/// # Where to define value objects
///
/// This trait is the contract only.  Concrete value objects belong in the
/// `api/<theme>/types/` directory of the theme that owns them — not in this
/// module.  For example, a `CommandId` goes in `api/command/types/`, not here.
/// `NonEmptyString` lives here solely as a reference implementation.
pub trait ValueObject: Eq + Hash + Clone + Send + Sync {}
