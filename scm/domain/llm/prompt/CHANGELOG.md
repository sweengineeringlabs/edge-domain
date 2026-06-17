# Changelog

All notable changes to `edge-llm-prompt` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-17

### Added
- Initial LLM Prompt domain primitive.
- `Prompt`, `ContextManager`, and `TokenCounter` trait contracts in `api/prompt/traits`.
- `PromptFactory` factory trait wiring the default prompt implementations.
- Type vocabulary: `Variable`, `VariableType`, `PromptMetadata`, `RenderContext`,
  `PromptCache`, and `PromptError`.
- Fluent builders: `VariableBuilder`, `PromptMetadataBuilder`, `PromptCacheBuilder`.
- Reference implementations `StaticPrompt`, `MapContextManager`,
  `HeuristicTokenCounter`, and `StdPromptFactory` in `core/prompt`.
