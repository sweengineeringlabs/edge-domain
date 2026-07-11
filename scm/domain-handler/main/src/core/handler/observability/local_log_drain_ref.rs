//! [`LocalLogDrainRef`] — wraps a borrowed real `LogDrain` reference as a local [`LogDrain`].

use edge_domain_observer as obs;

use super::into_handler_error::IntoHandlerError;
use crate::api::{HandlerError, LogDrain, LogEmitRequest, LogEmitResponse};

/// Adapter wrapping a borrowed real `LogDrain` reference as a local [`LogDrain`].
pub(super) struct LocalLogDrainRef<'a>(pub(super) &'a dyn obs::LogDrain);

impl LogDrain for LocalLogDrainRef<'_> {
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError> {
        obs::LogDrain::emit(
            self.0,
            obs::LogEmitRequest {
                level: req.level,
                handler_id: req.handler_id,
                message: req.message,
            },
        )
        .map(|_| LogEmitResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }
}
