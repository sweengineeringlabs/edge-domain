//! Test doubles for [`Step`](crate::api::Step) trait.
#![allow(dead_code)] // Test utilities, used in tests but not in main code path

pub(crate) mod always;
pub(crate) mod noop_step;
pub(crate) mod mutating_step;

// Re-export for tests (used in saf module tests)
pub(crate) use always::AlwaysPassStep;
