//! # edge_llm_provider
//!
//! LLM Provider domain primitive: execution backend abstraction for swappable LLM providers.
//!
//! Decouples agent orchestration from specific LLM backends (OpenAI, Claude, local models).

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use api::{
    ExecutionMode, ExecutionModel, ExecutionConfig, ExecutionError, ExecutionStepResult,
    TokenUsage, FinishReason, ModelInfo, ModelFamily, TokenizerAccuracy,
    LLMProvider, ProviderConfig, StreamChunk, StreamDelta, ToolCallDelta,
};
