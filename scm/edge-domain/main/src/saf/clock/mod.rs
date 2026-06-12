//! SAF — clock sub-module: clock and clock-factory facades.
mod clock_factory_svc;
mod clock_svc;
pub use self::clock_factory_svc::*;
pub use self::clock_svc::*;
