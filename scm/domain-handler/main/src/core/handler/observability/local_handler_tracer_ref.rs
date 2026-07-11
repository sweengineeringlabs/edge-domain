//! [`LocalHandlerTracerRef`] — wraps a borrowed real `HandlerTracer` reference as a local [`HandlerTracer`].

use edge_domain_observer as obs;

use super::into_handler_error::IntoHandlerError;
use super::local_span_adapter::LocalSpanAdapter;
use crate::api::{HandlerError, HandlerTracer, SpanStartRequest, SpanStartResponse};

/// Adapter wrapping a borrowed real `HandlerTracer` reference as a local [`HandlerTracer`].
pub(super) struct LocalHandlerTracerRef<'a>(pub(super) &'a dyn obs::HandlerTracer);

impl HandlerTracer for LocalHandlerTracerRef<'_> {
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError> {
        let resp = obs::HandlerTracer::start_span(
            self.0,
            obs::SpanStartRequest {
                handler_id: req.handler_id,
                operation: req.operation,
            },
        )
        .map_err(IntoHandlerError::into_handler_error)?;
        Ok(SpanStartResponse {
            span: Box::new(LocalSpanAdapter(resp.span)),
        })
    }
}
