//! [`CommandExecutionRequest`] — input for [`Command::execute`](crate::api::handler::traits::Command::execute).

/// Marker request; `execute` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default)]
pub struct CommandExecutionRequest;
