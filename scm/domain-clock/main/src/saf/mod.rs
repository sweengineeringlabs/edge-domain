mod clock;

pub use clock::{
    Clock, ClockError, ClockFactory, StdClockFactory, FixedClock, SystemClock,
    CLOCK_FACTORY_SVC, CLOCK_SVC,
};
