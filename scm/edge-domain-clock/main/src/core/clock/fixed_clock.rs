//! `Clock` impl for `FixedClock`.

use std::time::SystemTime;

use crate::api::clock::traits::Clock;
use crate::api::clock::types::FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> SystemTime {
        self.instant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_now_returns_pinned_instant() {
        let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(123);
        assert_eq!(FixedClock::new(instant).now(), instant);
    }
}
