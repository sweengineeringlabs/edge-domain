//! [`LocalCounterAdapter`] — wraps an owned real `Counter` as a local [`Counter`].

use edge_domain_observer as obs;

use super::into_handler_error::IntoHandlerError;
use crate::api::{Counter, HandlerError, IncrementRequest, IncrementResponse};

/// Adapter wrapping an owned real `Counter` as a local [`Counter`].
pub(super) struct LocalCounterAdapter(pub(super) Box<dyn obs::Counter>);

impl Counter for LocalCounterAdapter {
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        obs::Counter::increment(self.0.as_ref(), obs::IncrementRequest { delta: req.delta })
            .map(|_| IncrementResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}
