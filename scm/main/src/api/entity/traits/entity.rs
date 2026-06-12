//! `Entity` — identity-bearing domain object.

use std::hash::Hash;

/// A domain object with stable identity.
///
/// An entity has an [`Id`](Entity::Id) that uniquely identifies it within its
/// aggregate boundary.  Two entities are equal when their IDs are equal —
/// not when all their fields match (that is
/// [`ValueObject`](crate::api::valueobject::traits::value_object::ValueObject)).
///
/// Entities live *inside* aggregates.  The aggregate root is the consistency
/// boundary; entities it contains are not roots themselves.
///
/// # Relationship to Aggregate
///
/// [`Aggregate`](crate::api::event::traits::aggregate::Aggregate) is a
/// specialisation of `Entity` — every aggregate has identity.  The supertype
/// relationship (`Aggregate: Entity`) is deferred until the breaking-change
/// window; until then both traits coexist independently.
///
/// # Examples
///
/// ```rust
/// use edge_domain::Entity;
///
/// struct LineItem { id: u64, quantity: u32 }
///
/// impl Entity for LineItem {
///     type Id = u64;
///     fn id(&self) -> &u64 { &self.id }
/// }
/// ```
pub trait Entity: Send + Sync {
    /// The type that uniquely identifies this entity within its aggregate.
    type Id: Eq + Hash + Clone + Send + Sync;

    /// Return the entity's stable identifier.
    fn id(&self) -> &Self::Id;
}
