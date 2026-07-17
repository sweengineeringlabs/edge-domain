//! `CommandBus` — local decoupling boundary for dispatching commands.

use std::future::Future;
use std::pin::Pin;

use crate::api::handler::errors::HandlerError;
use crate::api::handler::command_bus_adapter::CommandBusAdapter;
use crate::api::handler::dto::CommandDispatchRequest;

/// Dispatches [`Command`](super::Command) instances to their executors.
///
/// Declared locally so `api/` never references `edge_application_command::CommandBus`
/// directly in a type position (SEA `no_foreign_type`). Any real `CommandBus`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait CommandBus: Send + Sync {
    /// Dispatch a command. Returns `Err` if execution fails.
    fn dispatch(
        &self,
        req: CommandDispatchRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>>;

    /// Wrap an already type-erased `&dyn ForeignCommandBus` reference so it
    /// can bridge into this trait via the blanket impl in `core/`.
    fn wrap<T: ?Sized>(inner: &T) -> CommandBusAdapter<'_, T>
    where
        Self: Sized,
    {
        CommandBusAdapter(inner)
    }
}
