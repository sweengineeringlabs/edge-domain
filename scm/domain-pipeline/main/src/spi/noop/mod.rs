//! Test doubles for [`Step`](crate::api::Step) trait.
#![allow(dead_code)] // Test utilities, used in tests but not in main code path

pub(crate) mod noop_step;
pub(crate) mod always_pass_step;
pub(crate) mod always_fail_step;
pub(crate) mod mutating_step;

pub(crate) use noop_step::NoopStep;
pub(crate) use always_pass_step::AlwaysPassStep;
pub(crate) use always_fail_step::AlwaysFailStep;
pub(crate) use mutating_step::MutatingStep;
