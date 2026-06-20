//! Provider domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ExecutionError;
pub use traits::{ExecutionModel, Provider, ProviderBootstrap, StreamHandler};
pub use types::{
    BufferedStreamHandler, CompletionInput, CompletionMessage, EchoProviderCompleter,
    EchoExecutionModel, ExecutionConfig, ExecutionMode, ExecutionStepResult, FinishReason,
    MessageRole, ModelFamily, ModelInfo, ProviderConfig, ProviderCore, StdProviderFactory,
    StreamChunk, StreamDelta, TokenUsage, TokenizerAccuracy, ToolCallDelta, ToolDefinition,
};
