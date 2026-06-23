//! Provider domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ExecutionError;
pub use traits::{ExecutionModel, Provider, ProviderBootstrap, StreamHandler};
pub use types::{
    BufferedStreamHandler, EchoExecutionModel,
    EchoProviderCompleter, ExecutionConfig, ExecutionMode, ExecutionStepResult, FinishReason,
    ModelFamily, ModelInfo, ProviderConfig, ProviderCore, StdProviderFactory,
    StreamChunk, StreamDelta, TokenUsage, TokenizerAccuracy, ToolCallDelta,
};
