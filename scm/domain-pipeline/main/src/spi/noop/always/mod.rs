//! Always-pattern test doubles — steps that follow predictable behavior.

pub(crate) mod fail_step;
pub(crate) mod pass_step;

pub(crate) use pass_step::AlwaysPassStep; // Re-exported by parent module noop
