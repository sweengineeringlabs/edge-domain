//! # edge-domain-projection
//!
//! The `Projection` port contract — CQRS read-model builder driven by domain events.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use edge_domain_event::DomainEvent;
pub use saf::StdProjectionFactory;
pub use saf::InMemoryProjection;
pub use saf::Projection;
pub use saf::ProjectionError;
pub use saf::ProjectionBootstrap;
