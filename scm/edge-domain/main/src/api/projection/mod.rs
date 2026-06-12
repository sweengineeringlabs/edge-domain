//! `Projection` theme — the read side of CQRS.
//!
//! Owns the [`Projection`] trait contract.  The in-memory reference projection
//! is obtained from
//! [`Domain::new_in_memory_projection`](crate::Domain::new_in_memory_projection),
//! which returns a `Box<dyn Projection>` — there is no public marker type (see
//! edge-domain#9).  Concrete projections belong in their owning theme and
//! implement [`Projection`] there.

pub mod traits;

pub use traits::Projection;
