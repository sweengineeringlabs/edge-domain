//! `PromptFactory` — constructor contract for the default prompt primitives.

use crate::api::prompt::types::{
    HeuristicTokenCounter, MapContextManager, PromptCacheBuilder, DefaultPrompt, PromptMetadata,
    PromptMetadataBuilder, StaticPrompt, StdPromptFactory, VariableBuilder,
};

/// Factory for the standard reference implementations.
///
/// Implement on any unit struct to gain the standard constructors.
pub trait PromptFactory {
    /// Return the standard prompt-factory instance.
    fn std_factory() -> StdPromptFactory {
        StdPromptFactory
    }

    /// Start a fluent [`VariableBuilder`].
    fn variable_builder() -> VariableBuilder {
        VariableBuilder::new()
    }

    /// Start a fluent [`PromptMetadataBuilder`].
    fn prompt_metadata_builder() -> PromptMetadataBuilder {
        PromptMetadataBuilder::new()
    }

    /// Start a fluent [`PromptCacheBuilder`].
    fn prompt_cache_builder() -> PromptCacheBuilder {
        PromptCacheBuilder::new()
    }

    /// Construct the reference [`StaticPrompt`] from a template body and metadata.
    fn prompt(template: String, metadata: PromptMetadata) -> StaticPrompt {
        StaticPrompt::new(template, metadata)
    }

    /// Construct an empty reference [`MapContextManager`].
    fn context_manager() -> MapContextManager {
        MapContextManager::new()
    }

    /// Construct the reference [`HeuristicTokenCounter`].
    fn token_counter() -> HeuristicTokenCounter {
        HeuristicTokenCounter::new()
    }

    /// Construct a pipeline [`DefaultPrompt`] (the connected Handler + Service
    /// face) over a reference prompt built from `template` and `metadata`.
    fn endpoint(template: String, metadata: PromptMetadata) -> DefaultPrompt {
        DefaultPrompt::new(StaticPrompt::new(template, metadata))
    }
}
