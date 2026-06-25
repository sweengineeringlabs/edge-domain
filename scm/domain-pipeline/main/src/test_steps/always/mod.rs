//! Always-pass and always-fail step test doubles.

pub(crate) mod fail_step;
pub(crate) mod pass_step;

pub(crate) use fail_step::AlwaysFailStep;
pub(crate) use pass_step::AlwaysPassStep;
