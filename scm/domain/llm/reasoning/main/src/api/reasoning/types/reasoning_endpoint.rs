//! `ReasoningEndpoint` — connects the reasoning primitive to the edge dispatch pipeline.
//!
//! Per ADR-037 this single type carries both faces of a connected native:
//! it implements `Handler` (register the domain + ride the dispatch pipeline)
//! and `Service` (typed, named consumption), with the `Service` face delegating
//! into the `Handler` (Service → Dispatch → Handler → core).

use crate::api::reasoning::types::LinearReasoning;

/// Pipeline endpoint for the reasoning primitive.
///
/// Wraps a [`LinearReasoning`] and exposes it as both a dispatchable
/// `Handler` and a typed `Service`.
#[derive(Clone, Debug)]
pub struct ReasoningEndpoint {
    pub(crate) reasoner: LinearReasoning,
}

impl ReasoningEndpoint {
    /// Construct an endpoint over the given reference reasoner.
    pub fn new(reasoner: LinearReasoning) -> Self {
        Self { reasoner }
    }
}
