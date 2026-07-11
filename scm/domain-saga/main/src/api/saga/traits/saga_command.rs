//! `SagaCommand` — local decoupling boundary for `Saga::Command`.

use std::future::Future;
use std::pin::Pin;

use crate::api::saga::errors::SagaError;
use crate::api::saga::types::SagaCommandDispatchRequest;

/// The minimal contract a [`Saga`](super::Saga)'s associated `Command` type must satisfy.
///
/// Declared locally so `api/` never references `edge_domain_command::Command`
/// directly in a type position (SEA `no_foreign_type`). Any `Command`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait SagaCommand: Send + Sync {
    /// Dispatch this command for execution.
    fn dispatch(
        &self,
        req: SagaCommandDispatchRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), SagaError>> + Send + '_>>;
}
