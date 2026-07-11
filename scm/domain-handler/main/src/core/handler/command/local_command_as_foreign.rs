//! [`LocalCommandAsForeign`] — wraps a local [`Command`](crate::api::Command) trait object as a
//! real `edge_domain_command::Command`, so it can flow through the real `CommandBus::dispatch`.

use edge_domain_command as cmd;

use crate::api::{Command, CommandExecutionRequest, CommandNameRequest};

/// Adapter wrapping a local [`Command`] trait object as a real [`cmd::Command`],
/// so it can flow through the real [`cmd::CommandBus::dispatch`].
pub(crate) struct LocalCommandAsForeign(pub(crate) Box<dyn Command>);

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
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), cmd::CommandError>> + Send + '_>,
    > {
        Box::pin(async move {
            self.0
                .execute(CommandExecutionRequest)
                .await
                .map_err(|e| cmd::CommandError::Internal(e.to_string()))
        })
    }
}
