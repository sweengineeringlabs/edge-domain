use crate::api::Histogram;

pub(crate) struct NoopHistogram;

impl Histogram for NoopHistogram {
    fn record(&self, value: f64) {
        let _ = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_value_discarded_happy() {
        let h = NoopHistogram;
        h.record(25.0);
        assert_eq!(std::mem::size_of_val(&h), 0);
    }

    #[test]
    fn test_record_zero_no_panic_error() {
        let h = NoopHistogram;
        h.record(0.0);
        assert_eq!(std::mem::size_of_val(&h), 0);
    }

    #[test]
    fn test_noop_histogram_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopHistogram>(), 0);
    }
}
