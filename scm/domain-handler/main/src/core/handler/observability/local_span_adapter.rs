//! [`LocalSpanAdapter`] — wraps an owned real `Span` as a local [`Span`].

use edge_application_observer as obs;

use super::into_handler_error::IntoHandlerError;
use crate::api::{
    HandlerError, Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse,
};

/// Adapter wrapping an owned real `Span` as a local [`Span`].
pub(super) struct LocalSpanAdapter(pub(super) Box<dyn obs::Span>);

impl Span for LocalSpanAdapter {
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        obs::Span::record(
            self.0.as_ref(),
            obs::SpanAnnotationRequest {
                key: req.key,
                value: req.value,
            },
        )
        .map(|_| SpanAnnotationResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }

    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        obs::Span::finish(self.0.as_ref(), obs::SpanFinishRequest)
            .map(|_| SpanFinishResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}
