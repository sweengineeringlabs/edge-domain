use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{Reasoning, ReasoningPattern, ThinkingProcess};
use crate::core::reasoning::default_reasoning::DefaultReasoningHandler;

pub use crate::api::LinearReasoning;
pub use crate::api::PatternMetadataBuilder;
pub use crate::api::ReasoningChainBuilder;
pub use crate::api::ReasoningFactory;
pub use crate::api::ReasoningStepBuilder;
pub use crate::api::StdReasoningFactory;
pub use crate::api::StepResultBuilder;
pub use crate::api::ThinkingProcessBuilder;

/// SAF contract identifier for the reasoning-factory service.
pub const REASONING_FACTORY_SVC: &str = "reasoning_factory";

/// Construct a dispatchable reasoning handler backed by the given reasoner.
pub fn reasoning_handler(
    reasoner: Arc<dyn Reasoning>,
) -> impl Handler<Request = String, Response = ThinkingProcess> {
    DefaultReasoningHandler { reasoner }
}

/// Construct a dispatchable reasoning handler backed by the reference [`LinearReasoning`].
pub fn default_reasoning_handler(
    pattern: ReasoningPattern,
) -> impl Handler<Request = String, Response = ThinkingProcess> {
    reasoning_handler(Arc::new(LinearReasoning::new(pattern)))
}
