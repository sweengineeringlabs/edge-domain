//! [`GaugeSetRequest`] — request to set a gauge's current value.

/// Request to set a [`Gauge`](crate::api::handler::traits::Gauge) to `value`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaugeSetRequest {
    /// The value to record.
    pub value: f64,
}
