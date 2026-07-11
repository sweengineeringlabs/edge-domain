//! [`LocalMetricRegistryRef`] — wraps a borrowed real `MetricRegistry` reference as a local [`MetricRegistry`].

use edge_domain_observer as obs;

use super::into_handler_error::IntoHandlerError;
use super::local_counter_adapter::LocalCounterAdapter;
use super::local_gauge_adapter::LocalGaugeAdapter;
use super::local_histogram_adapter::LocalHistogramAdapter;
use crate::api::{
    CounterLookupRequest, CounterLookupResponse, GaugeLookupRequest, GaugeLookupResponse,
    HandlerError, HistogramLookupRequest, HistogramLookupResponse, MetricRegistry,
};

/// Adapter wrapping a borrowed real `MetricRegistry` reference as a local [`MetricRegistry`].
pub(super) struct LocalMetricRegistryRef<'a>(pub(super) &'a dyn obs::MetricRegistry);

impl MetricRegistry for LocalMetricRegistryRef<'_> {
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, HandlerError> {
        let resp =
            obs::MetricRegistry::counter(self.0, obs::CounterLookupRequest { name: req.name })
                .map_err(IntoHandlerError::into_handler_error)?;
        Ok(CounterLookupResponse {
            counter: Box::new(LocalCounterAdapter(resp.counter)),
        })
    }

    fn histogram(
        &self,
        req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, HandlerError> {
        let resp =
            obs::MetricRegistry::histogram(self.0, obs::HistogramLookupRequest { name: req.name })
                .map_err(IntoHandlerError::into_handler_error)?;
        Ok(HistogramLookupResponse {
            histogram: Box::new(LocalHistogramAdapter(resp.histogram)),
        })
    }

    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, HandlerError> {
        let resp = obs::MetricRegistry::gauge(self.0, obs::GaugeLookupRequest { name: req.name })
            .map_err(IntoHandlerError::into_handler_error)?;
        Ok(GaugeLookupResponse {
            gauge: Box::new(LocalGaugeAdapter(resp.gauge)),
        })
    }
}
