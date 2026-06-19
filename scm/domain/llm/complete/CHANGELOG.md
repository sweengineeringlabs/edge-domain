# Changelog

All notable changes to `edge-llm-complete` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-19

### Added
- Initial LLM Complete domain primitive (ADR-043).
- `Completer` trait: blocking and streaming completion boundary.
- `CompleteFactory` factory trait with all-default constructor methods.
- Full type vocabulary: `CompletionRequest`, `CompletionResponse`, `Message`,
  `MessageContent`, `ContentPart`, `StreamChunk`, `StreamDelta`, `TokenUsage`,
  `ModelInfo`, `ToolDefinition`, `ToolCall`, `ToolCallDelta`, `ToolChoice`,
  `FinishReason`, `CacheControl`, `Role`, `ImageUrl`.
- Auxiliary traits: `CompleterHandler`, `CompleteOps`, `ContentFlattener`,
  `ModelOps`, `Processor`, `StreamOps`, `ToolOps`, `Validator`, `CacheableMessage`.
- `EchoCompleter` and `NoopCompleter` reference implementations.
- `StdCompleteFactory` zero-cost factory implementation.
