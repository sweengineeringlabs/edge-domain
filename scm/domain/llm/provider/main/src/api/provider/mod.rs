//! Provider domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ExecutionError;
pub use traits::{ExecutionModel, Provider, ProviderFactory, StreamHandler};
pub use types::{
    BufferedStreamHandler, EchoExecutionModel, ExecutionConfig, ExecutionMode, ExecutionStepResult,
    FinishReason, ModelFamily, ModelInfo, ProviderConfig, StaticProvider, StdProviderFactory,
    StreamChunk, StreamDelta, TokenUsage, TokenizerAccuracy, ToolCallDelta,
};
