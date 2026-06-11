//! `Saga` theme — orchestration of long-running business processes.
//!
//! Owns the [`Saga`] and [`SagaRegistry`] contracts, the [`SagaError`] type,
//! and the [`InMemorySagaRegistry`] reference marker.  Concrete sagas are
//! defined by consumers in their own bounded contexts and implement [`Saga`]
//! there.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::SagaError;
pub use traits::{Saga, SagaRegistry};
pub use types::InMemorySagaRegistry;
