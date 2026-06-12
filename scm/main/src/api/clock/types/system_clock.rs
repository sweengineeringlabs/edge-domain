//! `SystemClock` — wall-clock [`Clock`](crate::api::clock::traits::clock::Clock) implementation.

/// Delegates every [`now()`](crate::api::clock::traits::clock::Clock::now) call
/// to [`std::time::SystemTime::now`].
///
/// Use in production. For deterministic tests inject a
/// [`FixedClock`](crate::api::clock::types::fixed_clock::FixedClock) instead.
pub struct SystemClock;
