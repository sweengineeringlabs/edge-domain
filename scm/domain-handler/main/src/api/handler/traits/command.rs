//! `Command` — local decoupling boundary for a write operation dispatched via [`CommandBus`](super::CommandBus).

use std::future::Future;
use std::pin::Pin;

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{
    CommandExecutionRequest, CommandNameRequest, CommandNameResponse,
};

/// A named write operation that mutates domain state and returns no value.
///
/// Declared locally so `api/` never references `edge_application_command::Command`
/// directly in a type position (SEA `no_foreign_type`). Any real `Command`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait Command: Send + Sync {
    /// Stable name identifying this command type.
    fn name(&self, _req: CommandNameRequest) -> Result<CommandNameResponse, HandlerError> {
        Ok(CommandNameResponse {
            name: "command".to_string(),
        })
    }

    /// Execute the command, mutating domain state.
    fn execute(
        &self,
        req: CommandExecutionRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>>;
}
