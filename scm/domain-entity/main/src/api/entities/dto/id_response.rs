//! [`IdResponse`] — wrapper for an entity's identifier.

/// Result of [`Entity::id`](crate::api::Entity::id).
pub struct IdResponse<'a, I> {
    /// The entity's stable identifier.
    pub id: &'a I,
}
