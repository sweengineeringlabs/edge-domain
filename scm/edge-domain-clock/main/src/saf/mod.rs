//! SAF — clock service facade.

mod clock;

pub use crate::api::clock::Clock;
pub use crate::api::clock::ClockError;
pub use crate::api::clock::ClockFactory;
pub use crate::api::clock::FixedClock;
pub use crate::api::clock::SystemClock;
