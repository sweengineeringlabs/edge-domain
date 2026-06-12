//! # edge-domain-query
//!
//! The `Query` port contract — CQRS read-side with QueryBus and DirectQueryBus.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::DirectQueryBus;
pub use saf::Query;
pub use saf::QueryBus;
pub use saf::QueryBusFactory;
pub use saf::QueryError;
