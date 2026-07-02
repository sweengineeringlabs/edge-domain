//! `EchoExecutionModel` — reference [`ExecutionModel`](crate::api::provider::traits::ExecutionModel) implementation.

use crate::api::provider::types::ExecutionConfig;

/// Reference execution model that produces deterministic step results.
///
/// A domain primitive with no backend: it echoes its configuration and yields
/// a canned [`ExecutionStepResult`](crate::api::provider::types::ExecutionStepResult)
/// so the [`ExecutionModel`](crate::api::provider::traits::ExecutionModel)
/// contract can be exercised deterministically.
#[derive(Clone, Debug)]
pub struct EchoExecutionModel {
    pub(crate) config: ExecutionConfig,
}
