//! `Command` and `CommandBus` impls for noop types — both discard/no-op execution.

use futures::future::BoxFuture;

use crate::api::Command;
use crate::api::CommandBus;
use crate::api::CommandError;
use crate::api::NoopCommand;
use crate::api::NoopCommandBus;

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
