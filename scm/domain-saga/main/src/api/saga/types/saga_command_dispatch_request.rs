//! [`SagaCommandDispatchRequest`] — input for [`SagaCommand::dispatch`](crate::api::saga::traits::SagaCommand::dispatch).

/// Marker request; `dispatch` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default)]
pub struct SagaCommandDispatchRequest;
