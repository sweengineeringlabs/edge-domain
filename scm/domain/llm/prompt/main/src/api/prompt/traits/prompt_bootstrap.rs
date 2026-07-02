//! `PromptBootstrap` — constructor contract for the default prompt primitives.

use crate::api::prompt::errors::PromptError;
use crate::api::prompt::types::{
    CatalogTemplateProvider, HeuristicTokenCounter, MapContextManager, PromptBootstrapNameRequest,
    PromptBootstrapNameResponse, PromptCache, PromptCacheBuilder, PromptMetadata,
    PromptMetadataBuilder, PromptTemplateBuilder, StaticPrompt, StdPromptFactory, VariableBuilder,
};

/// Constructor namespace for the standard prompt reference implementations.
///
/// Implement on any unit struct to gain the standard constructors.
pub trait PromptBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(
        &self,
        _req: PromptBootstrapNameRequest,
    ) -> Result<PromptBootstrapNameResponse, PromptError> {
        Ok(PromptBootstrapNameResponse { name: "prompt" })
    }

    /// Return the standard prompt-factory instance.
    fn std_factory() -> StdPromptFactory
    where
        Self: Sized,
    {
        StdPromptFactory
    }

    /// Start a fluent [`VariableBuilder`].
    fn variable_builder() -> VariableBuilder
    where
        Self: Sized,
    {
        VariableBuilder::new()
    }

    /// Start a fluent [`PromptMetadataBuilder`].
    fn prompt_metadata_builder() -> PromptMetadataBuilder
    where
        Self: Sized,
    {
        PromptMetadataBuilder::new()
    }

    /// Start a fluent [`PromptCacheBuilder`].
    fn prompt_cache_builder() -> PromptCacheBuilder
    where
        Self: Sized,
    {
        PromptCacheBuilder::new()
    }

    /// Construct a [`PromptCache`] entry directly, without the builder.
    fn prompt_cache(key: String, rendered: String, token_count: usize) -> PromptCache
    where
        Self: Sized,
    {
        PromptCache::new(key, rendered, token_count)
    }

    /// Construct the reference [`StaticPrompt`] from a template body and metadata.
    fn prompt(template: String, metadata: PromptMetadata) -> StaticPrompt
    where
        Self: Sized,
    {
        StaticPrompt::new(template, metadata)
    }

    /// Construct an empty reference [`MapContextManager`].
    fn context_manager() -> MapContextManager
    where
        Self: Sized,
    {
        MapContextManager::new()
    }

    /// Construct the reference [`HeuristicTokenCounter`].
    fn token_counter() -> HeuristicTokenCounter
    where
        Self: Sized,
    {
        HeuristicTokenCounter::new()
    }

    /// Construct an empty reference [`CatalogTemplateProvider`].
    fn template_provider() -> CatalogTemplateProvider
    where
        Self: Sized,
    {
        CatalogTemplateProvider::new()
    }

    /// Start a fluent [`PromptTemplateBuilder`].
    fn prompt_template_builder() -> PromptTemplateBuilder
    where
        Self: Sized,
    {
        PromptTemplateBuilder::new()
    }
}
