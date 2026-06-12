//! `Clock` impl for `SystemClock`.

use std::time::SystemTime;

use crate::api::clock::traits::Clock;
use crate::api::clock::types::SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_advances_from_epoch() {
        assert!(SystemClock.now() > SystemTime::UNIX_EPOCH);
    }
}
