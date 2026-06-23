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
mod saf;

pub use saf::Registry;
pub use saf::RegistryBootstrap;
pub use saf::REGISTRY_FACTORY_SVC;
pub use saf::REGISTRY_SVC;
