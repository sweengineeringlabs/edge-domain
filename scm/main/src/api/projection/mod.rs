//! `Projection` theme — the read side of CQRS.
//!
//! Owns the [`Projection`] trait contract and the [`InMemoryProjection`]
//! reference marker.  Concrete projections that build a specific read model
//! belong in `api/<theme>/types/` of their owning theme and implement
//! [`Projection`] there.

pub mod traits;
pub mod types;

pub use traits::Projection;
pub use types::InMemoryProjection;
