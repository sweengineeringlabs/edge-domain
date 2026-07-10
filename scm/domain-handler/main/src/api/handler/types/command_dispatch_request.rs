//! [`CommandDispatchRequest`] — input for [`CommandBus::dispatch`](crate::api::handler::traits::CommandBus::dispatch).

use crate::api::handler::traits::Command;

/// Request to dispatch a [`Command`] through a [`CommandBus`](crate::api::handler::traits::CommandBus).
pub struct CommandDispatchRequest {
    /// The command to dispatch.
    pub command: Box<dyn Command>,
}
