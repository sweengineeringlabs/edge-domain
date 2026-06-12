pub mod fixed_clock;
pub mod system_clock;
pub mod traits;
pub mod types;
pub use traits::Clock;
pub use types::{FixedClock, SystemClock};
