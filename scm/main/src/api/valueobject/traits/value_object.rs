//! `ValueObject` marker trait.

use std::hash::Hash;

/// Marker trait for value objects.
///
/// A value object has no identity of its own — two instances are equal when
/// all their fields are equal.  Implementors must be equality-comparable,
/// hashable, cloneable, and safe to share across threads.
pub trait ValueObject: Eq + Hash + Clone + Send + Sync {}
