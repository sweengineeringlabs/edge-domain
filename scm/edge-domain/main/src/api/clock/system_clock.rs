//! `SystemClock` — type alias re-exporting from the designated struct home.
/// Wall-clock [`Clock`](crate::api::clock::traits::clock::Clock) backed by [`std::time::SystemTime::now`].
pub type SystemClock = crate::api::clock::types::SystemClock;
