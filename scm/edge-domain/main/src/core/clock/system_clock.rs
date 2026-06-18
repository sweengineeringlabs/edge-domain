//! [`Clock`] impl for [`SystemClock`].

use std::time::SystemTime;

use crate::api::Clock;
use crate::api::ClockFactory;
use crate::api::SystemClock;

#[expect(
    dead_code,
    reason = "SEA core/ structural anchor — pending swearchitect#84 and #85"
)]
pub(crate) struct DefaultSystemClock;

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

impl ClockFactory for DefaultSystemClock {}
