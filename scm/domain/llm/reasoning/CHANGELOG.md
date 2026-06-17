# Changelog

All notable changes to `edge-llm-reasoning` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-17

### Added
- Initial LLM Reasoning domain primitive.
- `Reasoning` primary trait contract in `api/reasoning/traits`.
- `ReasoningFactory` factory trait wiring the default reasoning implementation
  and the fluent value-type builders.
- Type vocabulary: `ReasoningPattern`, `ReasoningStep`, `StepResult`,
  `ThinkingProcess`, `PatternMetadata`, `ReasoningChain`, `ReasoningError`.
- `LinearReasoning` reference implementation in `core/reasoning`.
