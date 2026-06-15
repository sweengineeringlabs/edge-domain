//! # edge-domain-registry
//!
//! The `Registry` port contract — an id-keyed resolution registry of shared
//! entries, generalizing the handler/service/task-controller family.
//!
//! Register a shared entry under a string id and resolve it later. The
//! reference implementation is the in-process [`InMemoryRegistry`].

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;

pub use api::registry::InMemoryRegistry;
pub use api::registry::Registry;
pub use api::registry::RegistryError;
pub use api::registry::RegistryFactory;
pub use api::registry::StdRegistryFactory;
