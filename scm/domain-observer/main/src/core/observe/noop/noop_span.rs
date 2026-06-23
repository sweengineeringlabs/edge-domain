use crate::api::Span;

pub(crate) struct NoopSpan;

impl Span for NoopSpan {
    fn record(&self, key: &str, value: &str) {
        let _ = (key, value);
    }

    fn finish(&self) {
        let _ = self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_key_value_discarded_happy() {
        let s = NoopSpan;
        s.record("k", "v");
        assert_eq!(std::mem::size_of_val(&s), 0);
    }

    #[test]
    fn test_finish_completes_without_panic_error() {
        let s = NoopSpan;
        s.finish();
        assert_eq!(std::mem::size_of_val(&s), 0);
    }

    #[test]
    fn test_noop_span_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopSpan>(), 0);
    }
}
