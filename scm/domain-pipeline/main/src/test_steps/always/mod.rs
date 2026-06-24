//! Always-pass and always-fail step test doubles.

mod fail_step;
mod pass_step;

pub(crate) use fail_step::AlwaysFailStep;
pub(crate) use pass_step::AlwaysPassStep;
