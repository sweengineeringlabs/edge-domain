//! `DefaultProvider` — connects the provider primitive to the edge dispatch pipeline.
//!
//! Per ADR-037 this single type carries both faces of a connected native:
//! it implements `Handler` (register the domain + ride the dispatch pipeline)
//! and `Service` (typed, named consumption), with the `Service` face delegating
//! into the `Handler` (Service → Dispatch → Handler → core).

use crate::api::provider::types::EchoExecutionModel;

/// Pipeline endpoint for the provider primitive.
///
/// Wraps an [`EchoExecutionModel`] and exposes it as both a dispatchable
/// `Handler` and a typed `Service`.
#[derive(Clone, Debug)]
pub struct DefaultProvider {
    pub(crate) model: EchoExecutionModel,
}

impl DefaultProvider {
    /// Construct an endpoint over the given execution model.
    pub fn new(model: EchoExecutionModel) -> Self {
        Self { model }
    }
}
