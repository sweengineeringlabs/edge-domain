use crate::api::Gauge;

pub(crate) struct NoopGauge;

impl Gauge for NoopGauge {
    fn set(&self, value: f64) {
        let _ = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_value_discarded_happy() {
        let g = NoopGauge;
        g.set(42.0);
        assert_eq!(std::mem::size_of_val(&g), 0);
    }

    #[test]
    fn test_set_negative_value_no_panic_error() {
        let g = NoopGauge;
        g.set(-1.0);
        assert_eq!(std::mem::size_of_val(&g), 0);
    }

    #[test]
    fn test_noop_gauge_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopGauge>(), 0);
    }
}
