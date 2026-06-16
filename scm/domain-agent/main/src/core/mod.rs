//! No-op implementations for testing the agent contract.
//!
//! Real implementations live in plugins, not here.
//! This layer provides no-op implementations for contract testing;
//! production implementations (llmboot, state-machine, etc.) live in edge/plugins/*.

pub(crate) mod noop;
