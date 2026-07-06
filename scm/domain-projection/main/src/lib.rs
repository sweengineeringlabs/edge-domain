//! # edge-domain-projection
//!
//! The `Projection` port contract — CQRS read-model builder driven by domain events.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::InMemoryProjection;
pub use api::ProjectionApplyRequest;
pub use api::ProjectionError;
pub use api::ProjectionReadModelRequest;
pub use api::ProjectionReadModelResponse;
pub use api::TryDrainRequest;
pub use api::TryDrainResponse;
pub use edge_domain_event::DomainEvent;
pub use saf::{Projection, PROJECTION_SVC, PROJECTION_SVC_FACTORY};
