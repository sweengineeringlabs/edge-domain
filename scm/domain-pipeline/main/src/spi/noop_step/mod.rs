//! Test doubles for [`Step`](crate::api::Step) trait.

mod noop_step;
mod always_pass_step;
mod always_fail_step;
mod mutating_step;

pub use noop_step::NoopStep;
pub use always_pass_step::AlwaysPassStep;
pub use always_fail_step::AlwaysFailStep;
pub use mutating_step::MutatingStep;
