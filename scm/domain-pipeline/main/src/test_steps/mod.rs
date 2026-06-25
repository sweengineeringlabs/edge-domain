//! Test doubles and fixtures for pipeline testing.

pub(crate) mod always;
pub(crate) mod default_step;
pub(crate) mod mutating_step;

pub(crate) use always::{AlwaysFailStep, AlwaysPassStep};
pub(crate) use default_step::DefaultStep;
pub(crate) use mutating_step::MutatingStep;
