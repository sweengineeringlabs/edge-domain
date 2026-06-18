//! Provider domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ExecutionError;
pub use traits::{ExecutionModel, Provider, ProviderFactory, StreamHandler};
pub use types::{
    BufferedStreamHandler, EchoExecutionModel, ExecutionConfig, ExecutionConfigBuilder,
    ExecutionMode, ExecutionStepResult, FinishReason, ModelFamily, ModelInfo, ModelInfoBuilder,
    ProviderConfig, ProviderConfigBuilder, StaticProvider, StdProviderFactory, StreamChunk,
    StreamDelta, TokenUsage, TokenUsageBuilder, TokenizerAccuracy, ToolCallDelta,
    ToolCallDeltaBuilder,
};
