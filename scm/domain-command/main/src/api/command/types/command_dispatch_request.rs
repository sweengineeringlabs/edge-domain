//! [`CommandDispatchRequest`] — input for [`CommandBus::dispatch`](super::super::traits::CommandBus::dispatch).

use super::super::traits::Command;

/// Request to dispatch a [`Command`] through a [`CommandBus`](super::super::traits::CommandBus).
pub struct CommandDispatchRequest {
    /// The command to dispatch.
    pub command: Box<dyn Command>,
}
