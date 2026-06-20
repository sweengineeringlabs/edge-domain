//! Basic `Clock` usage example.

use std::time::{Duration, SystemTime};

use edge_domain_clock::{Clock, ClockBootstrap, FixedClock, SystemClock};

struct Clocks;
impl ClockBootstrap for Clocks {}

fn main() {
    let wall: SystemClock = Clocks::system();
    println!("now: {:?}", wall.now());

    let pinned: FixedClock = Clocks::fixed(SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000));
    println!("fixed: {:?}", pinned.now());
}
