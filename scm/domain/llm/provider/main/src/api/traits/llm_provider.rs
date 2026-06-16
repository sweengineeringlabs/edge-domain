use crate::api::types::{ProviderConfig, ExecutionError};
use async_trait::async_trait;

/// LLM provider abstraction (backend source: OpenAI, Claude, local, etc.)
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Provider name (e.g., "openai-gpt4", "anthropic-claude", "local-llama")
    fn name(&self) -> &str;

    /// Provider configuration
    fn config(&self) -> ProviderConfig;

    /// Execute prompt and return response
    async fn complete(&self, prompt: &str) -> Result<String, ExecutionError>;

    /// Check if provider is available and healthy
    async fn health_check(&self) -> Result<(), ExecutionError>;
}
