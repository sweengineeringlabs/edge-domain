//! `Spec<T>` — composable predicate for [`QueryableRepository`](super::queryable_repository::QueryableRepository) queries.

/// A composable predicate evaluated against domain entities.
///
/// Implementations encode a business rule that an entity either satisfies
/// or does not.  The repository evaluates specs against stored entities.
///
/// In-memory repositories evaluate specs in Rust.  Database-backed repositories
/// translate specs to SQL WHERE clauses or equivalent query DSL expressions.
///
/// ```rust,ignore
/// struct ActiveOrders;
///
/// impl Spec<Order> for ActiveOrders {
///     fn matches(&self, order: &Order) -> bool {
///         order.status == Status::Active
///     }
/// }
///
/// let active = repo.find_by(&ActiveOrders).await?;
/// ```
pub trait Spec<T: Send + Sync>: Send + Sync {
    /// Return `true` when `entity` satisfies this specification.
    fn matches(&self, _entity: &T) -> bool {
        false
    }
}
