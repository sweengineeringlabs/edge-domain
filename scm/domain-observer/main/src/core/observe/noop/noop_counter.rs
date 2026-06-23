use crate::api::Counter;

pub(crate) struct NoopCounter;

impl Counter for NoopCounter {
    fn increment(&self, delta: u64) {
        let _ = delta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_delta_discarded_happy() {
        let c = NoopCounter;
        c.increment(1);
        assert_eq!(std::mem::size_of_val(&c), 0);
    }

    #[test]
    fn test_increment_max_value_no_panic_error() {
        let c = NoopCounter;
        c.increment(u64::MAX);
        assert_eq!(std::mem::size_of_val(&c), 0);
    }

    #[test]
    fn test_noop_counter_is_zero_size_edge() {
        assert_eq!(std::mem::size_of::<NoopCounter>(), 0);
    }
}
