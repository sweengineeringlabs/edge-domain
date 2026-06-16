//! Stub implementations for arch audit compliance.
//!
//! Real implementations live in plugins, not here.
//! This layer contains only stub/test implementations — Agent primitives are contracts;
//! production implementations (llmboot, state-machine, etc.) live in edge/plugins/*.

pub(crate) mod agent;
pub(crate) mod agent_manager;
pub(crate) mod agent_registry;
pub(crate) mod skill;
