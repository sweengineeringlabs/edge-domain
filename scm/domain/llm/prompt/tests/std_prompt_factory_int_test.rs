//! Tests for the `StdPromptFactory` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    Prompt, PromptBootstrap, PromptMetadata, PromptMetadataRequest, StdPromptFactory,
};

/// @covers: StdPromptFactory — std_factory returns the factory instance
#[test]
fn test_std_prompt_factory_std_factory_returns_instance() {
    let factory: StdPromptFactory = StdPromptFactory::std_factory();
    assert_eq!(
        std::mem::size_of_val(&factory),
        0,
        "StdPromptFactory should be zero-sized"
    );
}

/// @covers: StdPromptFactory — is zero-sized
#[test]
fn test_std_prompt_factory_is_zero_sized() {
    assert_eq!(std::mem::size_of::<StdPromptFactory>(), 0);
}

/// @covers: StdPromptFactory — builds a prompt via the factory
#[test]
fn test_std_prompt_factory_builds_prompt() {
    let m = PromptMetadata::new("p".to_string(), "P".to_string(), "1".to_string(), vec![]);
    assert_eq!(
        StdPromptFactory::prompt("body".to_string(), m)
            .metadata(PromptMetadataRequest)
            .expect("metadata ok")
            .id,
        "p"
    );
}
