//! Blanket bridges from `edge_domain_command`'s traits to their local
//! `domain-handler` decoupling boundaries (SEA `no_foreign_type`).

use edge_domain_command as cmd;

use super::local_command_as_foreign::LocalCommandAsForeign;
use crate::api::HandlerError;
use crate::api::{Command, CommandBus, CommandDispatchRequest, CommandExecutionRequest};
use crate::api::{CommandNameRequest, CommandNameResponse};

/// Converts a real [`cmd::CommandError`] into the local [`HandlerError`].
trait IntoHandlerError {
    fn into_handler_error(self) -> HandlerError;
}

impl IntoHandlerError for cmd::CommandError {
    fn into_handler_error(self) -> HandlerError {
        HandlerError::ExecutionFailed(self.to_string())
    }
}

impl<T: cmd::Command + ?Sized> Command for T {
    fn name(&self, _req: CommandNameRequest) -> Result<CommandNameResponse, HandlerError> {
        cmd::Command::name(self, cmd::NameRequest)
            .map(|r| CommandNameResponse { name: r.name })
            .map_err(IntoHandlerError::into_handler_error)
    }

    fn execute(
        &self,
        _req: CommandExecutionRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>>
    {
        Box::pin(async move {
            cmd::Command::execute(self, cmd::ExecutionRequest)
                .await
                .map_err(IntoHandlerError::into_handler_error)
        })
    }
}

impl<T: cmd::CommandBus + ?Sized> CommandBus for T {
    fn dispatch(
        &self,
        req: CommandDispatchRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>>
    {
        let command = req.command;
        Box::pin(async move {
            cmd::CommandBus::dispatch(
                self,
                cmd::CommandDispatchRequest {
                    command: Box::new(LocalCommandAsForeign(command)),
                },
            )
            .await
            .map_err(IntoHandlerError::into_handler_error)
        })
    }
}

impl<T: cmd::CommandBus + ?Sized> CommandBus for crate::api::CommandBusAdapter<'_, T> {
    fn dispatch(
        &self,
        req: CommandDispatchRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>>
    {
        CommandBus::dispatch(self.0, req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_handler_error_converts_command_error_message_happy() {
        let err = cmd::CommandError::Internal("boom".to_string());
        assert_eq!(
            err.into_handler_error(),
            HandlerError::ExecutionFailed("internal: boom".to_string())
        );
    }
}
