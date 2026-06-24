//! Test doubles and fixtures for pipeline testing.

mod always_fail_step;
mod always_pass_step;
mod mutating_step;

pub(crate) use always_fail_step::AlwaysFailStep;
pub(crate) use always_pass_step::AlwaysPassStep;
pub(crate) use mutating_step::MutatingStep;
