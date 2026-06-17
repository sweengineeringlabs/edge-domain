//! SAF re-exports for the no-op contract implementations.
//!
//! These zero-sized types implement the agent contracts with no-op behaviour.
//! They exist for contract testing and as safe defaults before a real plugin
//! implementation is wired in.

pub use crate::api::{
    NoopAgent, NoopAgentLifecycle, NoopAgentManager, NoopAgentRegistry, NoopSchemaValidator,
    NoopSkill, NoopValidator,
};
