//! `Command` and `CommandBus` impls for noop types — both discard/no-op execution.

use futures::future::BoxFuture;

use crate::api::command::traits::Command;
use crate::api::command::types::NoopCommand;
use crate::api::command::types::NoopCommandBus;
use crate::api::command::CommandBus;
use crate::api::command::CommandError;

impl Command for NoopCommand {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

impl CommandBus for NoopCommandBus {
    fn dispatch(&self, _: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}
