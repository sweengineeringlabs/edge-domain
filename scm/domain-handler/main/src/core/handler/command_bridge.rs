//! Blanket bridges from `edge_domain_command`'s traits to their local
//! `domain-handler` decoupling boundaries (SEA `no_foreign_type`).

use edge_domain_command as cmd;

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
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), HandlerError>> + Send + '_>> {
        CommandBus::dispatch(self.0, req)
    }
}

/// Adapter wrapping a local [`Command`] trait object as a real [`cmd::Command`],
/// so it can flow through the real [`cmd::CommandBus::dispatch`].
struct LocalCommandAsForeign(Box<dyn Command>);

impl cmd::Command for LocalCommandAsForeign {
    fn name(&self, _req: cmd::NameRequest) -> Result<cmd::NameResponse, cmd::CommandError> {
        self.0
            .name(CommandNameRequest)
            .map(|r| cmd::NameResponse { name: r.name })
            .map_err(|e| cmd::CommandError::Internal(e.to_string()))
    }

    fn execute(
        &self,
        _req: cmd::ExecutionRequest,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), cmd::CommandError>> + Send + '_>> {
        Box::pin(async move {
            self.0
                .execute(CommandExecutionRequest)
                .await
                .map_err(|e| cmd::CommandError::Internal(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;

    struct OkCmd;
    impl cmd::Command for OkCmd {
        fn execute(
            &self,
            _req: cmd::ExecutionRequest,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), cmd::CommandError>> + Send + '_>>
        {
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn test_command_bridge_name_default_returns_command_happy() {
        assert_eq!(Command::name(&OkCmd, CommandNameRequest).unwrap().name, "command");
    }

    #[test]
    fn test_command_bridge_execute_ok_returns_ok_error() {
        assert_eq!(block_on(Command::execute(&OkCmd, CommandExecutionRequest)), Ok(()));
    }

    #[test]
    fn test_command_bus_bridge_dispatches_via_real_bus_edge() {
        let bus = cmd::DirectCommandBus;
        let result = block_on(CommandBus::dispatch(
            &bus,
            CommandDispatchRequest {
                command: Box::new(OkCmd),
            },
        ));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_command_bus_adapter_bridges_erased_reference_edge() {
        let bus = cmd::DirectCommandBus;
        let erased: &dyn cmd::CommandBus = &bus;
        let adapter = crate::api::CommandBusAdapter(erased);
        let result = block_on(CommandBus::dispatch(
            &adapter,
            CommandDispatchRequest {
                command: Box::new(OkCmd),
            },
        ));
        assert_eq!(result, Ok(()));
    }
}
