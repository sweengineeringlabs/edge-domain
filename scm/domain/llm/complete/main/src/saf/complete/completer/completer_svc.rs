pub use crate::api::CacheControl;
pub use crate::api::CompleteError;
pub use crate::api::Completer;
pub use crate::api::CompletionRequest;
pub use crate::api::CompletionResponse;
pub use crate::api::ContentPart;
pub use crate::api::EchoCompleter;
pub use crate::api::FinishReason;
pub use crate::api::ImageUrl;
pub use crate::api::Message;
pub use crate::api::MessageContent;
pub use crate::api::ModelInfo;
pub use crate::api::NoopCompleter;
pub use crate::api::Role;
pub use crate::api::StreamChunk;
pub use crate::api::StreamDelta;
pub use crate::api::TokenUsage;
pub use crate::api::ToolCall;
pub use crate::api::ToolCallDelta;
pub use crate::api::ToolChoice;
pub use crate::api::ToolDefinition;

/// SAF contract identifier for the completer service.
pub const COMPLETER_SVC: &str = "completer";
