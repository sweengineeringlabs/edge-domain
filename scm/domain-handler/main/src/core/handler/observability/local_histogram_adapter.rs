//! [`LocalHistogramAdapter`] — wraps an owned real `Histogram` as a local [`Histogram`].

use edge_domain_observer as obs;

use super::into_handler_error::IntoHandlerError;
use crate::api::{HandlerError, Histogram, HistogramRecordRequest, HistogramRecordResponse};

/// Adapter wrapping an owned real `Histogram` as a local [`Histogram`].
pub(super) struct LocalHistogramAdapter(pub(super) Box<dyn obs::Histogram>);

impl Histogram for LocalHistogramAdapter {
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, HandlerError> {
        obs::Histogram::record(
            self.0.as_ref(),
            obs::HistogramRecordRequest { value: req.value },
        )
        .map(|_| HistogramRecordResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }
}
