//! Test doubles and fixtures for pipeline testing.

mod always;
mod mutating_step;

pub(crate) use always::{AlwaysFailStep, AlwaysPassStep};
pub(crate) use mutating_step::MutatingStep;
