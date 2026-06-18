use edge_domain_command::{Command, CommandError};
use futures::future::BoxFuture;

use crate::api::NoopSagaCommand;

impl Command for NoopSagaCommand {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}
