//! Test double: counter for testing mutations.

/// A simple counter used in mutation tests.
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub(crate) struct Counter {
    /// The count value.
    pub(crate) count: usize,
}

impl Counter {
    /// Create a new counter starting at 0.
    pub(crate) fn new() -> Self {
        Self { count: 0 }
    }

    /// Increment the counter by 1.
    fn increment(&mut self) {
        self.count += 1;
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_happy_creates_zero() {
        let counter = Counter::new();
        assert_eq!(counter.count, 0);
    }

    #[test]
    fn test_increment_happy_adds_one() {
        let mut counter = Counter::new();
        counter.increment();
        assert_eq!(counter.count, 1);
    }

    #[test]
    fn test_default_happy_creates_zero() {
        let counter = Counter::default();
        assert_eq!(counter.count, 0);
    }
}
