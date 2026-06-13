//! `Command` impl for [`NoopCommand`].

use futures::future::BoxFuture;

use crate::api::command::traits::Command;
use crate::api::command::types::NoopCommand;
use crate::api::command::CommandError;

impl Command for NoopCommand {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}
