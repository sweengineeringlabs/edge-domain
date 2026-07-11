//! [`LocalGaugeAdapter`] — wraps an owned real `Gauge` as a local [`Gauge`].

use edge_domain_observer as obs;

use super::into_handler_error::IntoHandlerError;
use crate::api::{Gauge, GaugeSetRequest, GaugeSetResponse, HandlerError};

/// Adapter wrapping an owned real `Gauge` as a local [`Gauge`].
pub(super) struct LocalGaugeAdapter(pub(super) Box<dyn obs::Gauge>);

impl Gauge for LocalGaugeAdapter {
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError> {
        obs::Gauge::set(self.0.as_ref(), obs::GaugeSetRequest { value: req.value })
            .map(|_| GaugeSetResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}
