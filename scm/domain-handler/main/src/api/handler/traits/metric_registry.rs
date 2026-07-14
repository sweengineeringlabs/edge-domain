//! `MetricRegistry` — local decoupling boundary for a metric instrument factory.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{
    CounterLookupRequest, CounterLookupResponse, GaugeLookupRequest, GaugeLookupResponse,
    HistogramLookupRequest, HistogramLookupResponse,
};

/// Creates named metric instruments.
///
/// Declared locally so `api/` never references `edge_application_observer::MetricRegistry`
/// directly in a type position (SEA `no_foreign_type`). Any real `MetricRegistry`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait MetricRegistry: Send + Sync {
    /// Return a counter for `name`.
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, HandlerError>;

    /// Return a histogram for `name`.
    fn histogram(
        &self,
        req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, HandlerError>;

    /// Return a gauge for `name`.
    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, HandlerError>;
}
