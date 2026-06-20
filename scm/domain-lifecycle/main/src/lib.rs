//! # edge-domain-lifecycle
//!
//! The `Lifecycle` port contract — a generic state-machine abstraction with a
//! pluggable [`TransitionPolicy`].
//!
//! Model the lifecycle of any entity (task, connection, saga, actor) without
//! coupling the framework to a specific state vocabulary.  The reference
//! implementation is [`ManagedLifecycle`], driven by [`PermissivePolicy`] or a
//! consumer-supplied policy.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Lifecycle;
pub use saf::LifecycleError;
pub use saf::LifecycleBootstrap;
pub use saf::ManagedLifecycle;
pub use saf::PermissivePolicy;
pub use saf::StdLifecycleFactory;
pub use saf::TransitionPolicy;
pub use saf::LIFECYCLE_FACTORY_SVC;
pub use saf::LIFECYCLE_SVC;
pub use saf::TRANSITION_POLICY_SVC;
