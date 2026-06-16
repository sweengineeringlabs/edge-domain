//! LLM Agent Domain (`edge_llm_agent`)
//!
//! Primary agent primitive for LLM systems: skill orchestrator with explicit lifecycle
//! state machine, multi-modal messaging, and tool governance.

pub mod api;
pub mod core;
pub mod saf;

pub use api::*;
