//! `Saga` theme — orchestration of long-running business processes.
//!
//! Owns the [`Saga`] and [`SagaStore`] contracts and the [`SagaError`] type.
//! The in-memory reference store is obtained from
//! [`Domain::new_in_memory_saga_store`](crate::Domain::new_in_memory_saga_store),
//! which returns a `Box<dyn SagaStore>` — there is no public marker type
//! (see edge-domain#9).  Concrete sagas are defined by consumers in their own
//! bounded contexts and implement [`Saga`] there.

pub mod errors;
pub mod traits;

pub use errors::SagaError;
pub use traits::{Saga, SagaStore};
