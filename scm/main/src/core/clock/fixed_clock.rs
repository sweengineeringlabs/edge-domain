//! [`Clock`] impl for [`FixedClock`].

use std::time::SystemTime;

use crate::api::clock::traits::Clock;
use crate::api::clock::types::FixedClock;

#[expect(
    dead_code,
    reason = "SEA core/ structural anchor — pending swearchitect#84 and #85"
)]
pub(crate) struct DefaultFixedClock;

impl Clock for FixedClock {
    fn now(&self) -> SystemTime {
        self.instant
    }
}
