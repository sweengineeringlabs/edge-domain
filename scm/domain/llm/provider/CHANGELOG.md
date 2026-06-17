# Changelog

All notable changes to `edge-llm-provider` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-17

### Added
- Initial LLM Provider domain primitive (ADR-033).
- `LLMProvider` and `ExecutionModel` trait contracts in `api/provider/traits`.
- `ProviderFactory` factory trait wiring the default provider implementation.
- Type vocabulary: `ExecutionMode`, `TokenUsage`, `ExecutionError`, `ExecutionStepResult`,
  `ExecutionConfig`, `ProviderConfig`, `StreamChunk`, `StreamDelta`, `ToolCallDelta`,
  `FinishReason`, `ModelInfo`, `ModelFamily`, `TokenizerAccuracy`.
- `DefaultProvider` reference implementation in `core/provider`.
