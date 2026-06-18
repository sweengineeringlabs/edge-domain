use crate::api::Counter;
use crate::api::Gauge;
use crate::api::Histogram;
use crate::api::MetricRegistry;
use super::noop_counter::NoopCounter;
use super::noop_gauge::NoopGauge;
use super::noop_histogram::NoopHistogram;

pub(crate) struct NoopMetricRegistry;

impl NoopMetricRegistry {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl MetricRegistry for NoopMetricRegistry {
    fn counter(&self, name: &str) -> Box<dyn Counter> {
        let _ = name;
        Box::new(NoopCounter)
    }

    fn histogram(&self, name: &str) -> Box<dyn Histogram> {
        let _ = name;
        Box::new(NoopHistogram)
    }

    fn gauge(&self, name: &str) -> Box<dyn Gauge> {
        let _ = name;
        Box::new(NoopGauge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_noop_metric_registry_happy() {
        let _ = NoopMetricRegistry::new();
    }

    #[test]
    fn test_counter_returns_usable_counter_error() {
        let r = NoopMetricRegistry::new();
        r.counter("errs").increment(u64::MAX);
    }

    #[test]
    fn test_all_three_primitives_constructible_edge() {
        let r = NoopMetricRegistry::new();
        r.counter("c").increment(1);
        r.histogram("h").record(1.0);
        r.gauge("g").set(1.0);
    }
}
