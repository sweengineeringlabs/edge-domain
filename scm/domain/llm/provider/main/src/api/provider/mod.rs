//! Provider domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ExecutionError;
pub use traits::{ExecutionModel, Provider, ProviderFactory, StreamHandler};
pub use types::{
    BufferedStreamHandler, CompletionInput, CompletionMessage, EchoProviderCompleter,
    EchoExecutionModel, ExecutionConfig, ExecutionMode, ExecutionStepResult, FinishReason,
    MessageRole, ModelFamily, ModelInfo, ProviderConfig, StaticProvider, StdProviderFactory,
    StreamChunk, StreamDelta, TokenUsage, TokenizerAccuracy, ToolCallDelta, ToolDefinition,
};
