//! Basic `edge-llm-provider` usage example.

use edge_llm_provider::{
    ExecutionMode, ModelFamily, ModelInfo, Provider, ProviderConfig, ProviderFactory,
    StdProviderFactory,
};

fn main() {
    let config = ProviderConfig::new("claude".to_string(), 0.7, 200_000);
    let info = ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        200_000,
    );

    let provider = StdProviderFactory::provider(config, info);
    println!("provider: {}", provider.name());
    println!("family: {:?}", provider.model_family());
    println!("healthy: {:?}", provider.health_check().is_ok());

    let mode = ExecutionMode::Streaming;
    println!("streaming mode: {}", mode.is_streaming());
}
